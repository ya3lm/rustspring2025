# Financial Data Fetcher in Rust

A robust Rust application that periodically fetches and records pricing data for Bitcoin, Ethereum, and the S&P 500 index.



## Features

- **Real-time Price Tracking**: Fetches current prices every 10 seconds
- **Multiple Data Sources**:
  - CoinGecko API for cryptocurrency prices
  - Yahoo Finance API for S&P 500 index
- **Automatic Rate Limit Handling**: Pauses when API limits are reached
- **Persistent Storage**: Saves data to separate text files
- **Error Resilience**: Continues operation even if one API fails
