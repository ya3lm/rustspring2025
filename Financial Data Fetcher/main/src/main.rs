use serde::Deserialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use ureq;

// 1. STRUCTS
#[derive(Debug)]
struct Bitcoin {
    last_price: Option<f64>,
}

#[derive(Debug)]
struct Ethereum {
    last_price: Option<f64>,
}

#[derive(Debug)]
struct SP500 {
    last_price: Option<f64>,
}

// 2. TRAIT IMPLEMENTATION
trait Pricing {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>>;
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>>;
    fn display(&self) -> String;
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct ApiResponse {
            bitcoin: BitcoinPrice,
        }
        #[derive(Deserialize)]
        struct BitcoinPrice {
            usd: f64,
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
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
        let price = response.bitcoin.usd;
        self.last_price = Some(price);
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bitcoin_prices.txt")?;
        writeln!(file, "{}", price)?;
        Ok(())
    }

    fn display(&self) -> String {
        format!("Bitcoin: ${:.2}", self.last_price.unwrap_or(0.0))
    }
}

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

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Using Yahoo Finance as a free alternative
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        
        #[derive(Deserialize)]
        struct YahooResponse {
            chart: Chart,
        }
        
        #[derive(Deserialize)]
        struct Chart {
            result: Vec<ChartResult>,
        }
        
        #[derive(Deserialize)]
        struct ChartResult {
            meta: Meta,
        }
        
        #[derive(Deserialize)]
        struct Meta {
            #[serde(rename = "regularMarketPrice")]
            regular_market_price: f64,
        }

        let response = ureq::get(url)
            .call()?
            .into_json::<YahooResponse>()?;
            
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

fn main() {
    // Initialize assets
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { last_price: None }),
        Box::new(Ethereum { last_price: None }),
        Box::new(SP500 { last_price: None }),
    ];

    // Main loop
    loop {
        println!("\nFetching new prices...");
        for asset in &mut assets {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("{}", asset.display());
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving price: {}", e);
                    }
                }
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
            thread::sleep(Duration::from_secs(2)); // Add small delay between requests
        }

        println!("Waiting 10 seconds before next fetch...");
        thread::sleep(Duration::from_secs(10));
    }
}