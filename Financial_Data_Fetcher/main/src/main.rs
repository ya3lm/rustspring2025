// Import necessary libraries
use serde::Deserialize;  // For JSON deserialization
use std::error::Error;   // For error handling
use std::fs::OpenOptions; // For file operations
use std::io::Write;      // For writing to files
use std::thread;         // For sleep functionality
use std::time::Duration; // For time intervals
use ureq;               // For HTTP requests

// 1. STRUCT DEFINITIONS
// Each struct represents a financial asset and stores its last fetched price

/// Bitcoin asset with price tracking
#[derive(Debug)]  // Enable debug printing
struct Bitcoin {
    last_price: Option<f64>,  // Stores the most recent price (None if not fetched yet)
}

/// Ethereum asset with price tracking
#[derive(Debug)]
struct Ethereum {
    last_price: Option<f64>,
}

/// S&P 500 index with price tracking
#[derive(Debug)]
struct SP500 {
    last_price: Option<f64>,
}

// 2. PRICING TRAIT AND IMPLEMENTATIONS
/// Defines common behavior for all priceable assets
trait Pricing {
    /// Fetches the current price from API
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>>;
    
    /// Saves the price to a file
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>>;
    
    /// Formats the price for display
    fn display(&self) -> String;
}

// Bitcoin implementation
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Nested structs for JSON parsing
        #[derive(Deserialize)]
        struct ApiResponse {
            bitcoin: BitcoinPrice,  // Matches CoinGecko's response structure
        }
        #[derive(Deserialize)]
        struct BitcoinPrice {
            usd: f64,  // Price in USD
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        
        // Make HTTP request with rate limit handling
        let response = match ureq::get(url).call() {
            Ok(resp) => resp,
            // If rate limited (429), wait 60 seconds and retry
            Err(ureq::Error::Status(code, _)) if code == 429 => {
                println!("Rate limited, waiting 60 seconds...");
                thread::sleep(Duration::from_secs(60));
                ureq::get(url).call()?
            }
            Err(e) => return Err(Box::new(e)),  // Propagate other errors
        };

        // Parse JSON response and extract price
        let response = response.into_json::<ApiResponse>()?;
        let price = response.bitcoin.usd;
        self.last_price = Some(price);  // Update last known price
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        // Open file in append mode, create if doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bitcoin_prices.txt")?;
            
        // Write price to file
        writeln!(file, "{}", price)?;
        Ok(())
    }

    fn display(&self) -> String {
        // Format price with 2 decimal places, default to 0.0 if None
        format!("Bitcoin: ${:.2}", self.last_price.unwrap_or(0.0))
    }
}

// Ethereum implementation (similar structure to Bitcoin)
impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct ApiResponse {
            ethereum: EthereumPrice,
        }
        #[derive(Deserialize)]
        struct EthereumPrice {
            usd: f64,
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        
        // Same rate limit handling as Bitcoin
        let response = match ureq::get(url).call() {
            Ok(resp) => resp,
            Err(ureq::Error::Status(code, _)) if code == 429 => {
                println!("Rate limited, waiting 60 seconds...");
                thread::sleep(Duration::from_secs(60));
                ureq::get(url).call()?
            }
            Err(e) => return Err(Box::new(e)),
        };

        let response = response.into_json::<ApiResponse>()?;
        let price = response.ethereum.usd;
        self.last_price = Some(price);
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ethereum_prices.txt")?;
        writeln!(file, "{}", price)?;
        Ok(())
    }

    fn display(&self) -> String {
        format!("Ethereum: ${:.2}", self.last_price.unwrap_or(0.0))
    }
}

// S&P 500 implementation (uses Yahoo Finance API)
impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Yahoo Finance API endpoint for S&P 500 (^GSPC)
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        
        // Complex nested structures to match Yahoo's JSON response
        #[derive(Deserialize)]
        struct YahooResponse {
            chart: Chart,
        }
        
        #[derive(Deserialize)]
        struct Chart {
            result: Vec<ChartResult>,  // Array of results (we take first)
        }
        
        #[derive(Deserialize)]
        struct ChartResult {
            meta: Meta,  // Metadata containing price
        }
        
        #[derive(Deserialize)]
        struct Meta {
            #[serde(rename = "regularMarketPrice")]  // Map JSON field to Rust naming
            regular_market_price: f64,
        }

        // Make request and parse JSON
        let response = ureq::get(url)
            .call()?
            .into_json::<YahooResponse>()?;
            
        // Extract price from nested structure
        let price = response.chart.result[0].meta.regular_market_price;
        self.last_price = Some(price);
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("sp500_prices.txt")?;
        writeln!(file, "{}", price)?;
        Ok(())
    }

    fn display(&self) -> String {
        format!("S&P 500: ${:.2}", self.last_price.unwrap_or(0.0))
    }
}

/// Main entry point of the application
fn main() {
    // Initialize our three assets with no known price yet
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { last_price: None }),
        Box::new(Ethereum { last_price: None }),
        Box::new(SP500 { last_price: None }),
    ];

    // Main program loop - runs indefinitely
    loop {
        println!("\nFetching new prices...");
        
        // Process each asset in sequence
        for asset in &mut assets {
            match asset.fetch_price() {
                Ok(price) => {
                    // Display and save successful fetches
                    println!("{}", asset.display());
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving price: {}", e);
                    }
                }
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
            // Small delay between assets to avoid rate limiting
            thread::sleep(Duration::from_secs(2));
        }

        // Wait 10 seconds before next full cycle
        println!("Waiting 10 seconds before next fetch...");
        thread::sleep(Duration::from_secs(10));
    }
}