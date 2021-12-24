use yahoo_finance_api as yahoo;
//use std::time::{Duration, UNIX_EPOCH};
use chrono::{Utc, TimeZone};
use std::process::exit;

fn main() {
    let ticker = "AAPL";
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);

    let reply = provider.get_quote_history(ticker, start, end);
    let response = match reply {
        Err(e) => {
            eprintln!("Unable to optain quotes for {} due to '{}'", ticker, e);
            exit(1);
        },
        Ok(r) => r
    };
    
    let quotes = response.quotes().unwrap();

    println!("Apple's quotes in January: {:?}", quotes);
}
