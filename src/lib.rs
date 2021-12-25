// lib.rs
//
// . library to load stock price informaiton from the yahoo api
//
use yahoo_finance_api as yahoo;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
struct Quote {
    timestamp: i64,
    high: f32,
    low: f32,
    open: f32,
    close: f32,
    adjclose: f32,
    volume: u64
}

#[derive(Debug)]
pub struct PriceSeries {
    ticker: String,
    quotes: Box<[Quote]>,
    start: DateTime<Utc>,
    end: DateTime<Utc>
}

impl PriceSeries {
    pub fn new(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<PriceSeries, yahoo::YahooError> {
        Ok(PriceSeries{
            ticker: String::from(ticker),
            quotes: Box::new([
                Quote{
                    timestamp: 1,
                    high: 12.0,
                    low: 8.0,
                    open: 10.0,
                    close: 10.0,
                    adjclose: 10.0,
                    volume: 12
                }
            ]),
            start,
            end 
        })
    }

}


    // let ticker = "AAPL";
    // let provider = yahoo::YahooConnector::new();
    // let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    // let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);

    // let reply = provider.get_quote_history(ticker, start, end);
    // let response = match reply {
    //     Err(e) => {
    //         eprintln!("Unable to obtain quotes for {} due to '{}'", ticker, e);
    //         exit(1);
    //     },
    //     Ok(r) => r
    // };
    
    // let quotes = response.quotes().unwrap();
