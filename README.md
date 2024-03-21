# Twelvedata-rs
A convenient Rust wrapper for the [Twelve Data API](https://twelvedata.com/docs#getting-started).

# !!! NOT PRODUCTION READY (yet) !!!
I have work to do. This is only on crates.io because I wanted to be sure that the name is secured.

**Changes are happening extremely rapidly. Code that worked yesterday might not work today. For example, at the time of writing I am reimplementing everything to a utilize a builder pattern. Once that is done I think I am gonna ditch reqwest and move everything to a http client like hyper.**

## Features
- [x] Reference Data
    - [x] Stocks List
    - [x] Forex Pairs List
    - [x] Cryptocurrencies List
    - [x] ETF List
    - [x] Indices List
    - [x] Funds List
    - [x] Bonds List
    - [x] Exchanges
    - [x] Cryptocurrency Exchanges
    - [ ] Technical Indicators Interface (look at the twelvedata [docs](https://twelvedata.com/docs#technical-indicators-interface) and you will understand...yikes)
- [x] Core Data
    - [x] Time Series
    - [x] Exchange Rate
    - [x] Currency Conversion
    - [x] Quote
    - [x] Real-Time Price
    - [x] End of Day Price
    - [x] Market Movers
- [] Mutual Funds
- [] Fundmentals
- [] Analysis
- [] WebSocket
- [] Advanced
- [] Technical Indicators