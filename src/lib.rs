// lib.rs
//
// . library to load stock price information from the yahoo api
//

use yahoo_finance_api::{Quote, YahooError, YahooConnector, YResponse};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct PriceSeries {
    ticker: String,
    quotes: Vec<Quote>,
    start: DateTime<Utc>,
    end: DateTime<Utc>
}

impl PriceSeries {
    pub fn new(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<PriceSeries, YahooError> {
        Ok(PriceSeries{
            ticker: String::from(ticker),
            quotes: Vec::new(),
            start,
            end 
        })

    }

    pub fn from_range(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> 
        Result<PriceSeries, YahooError> {
        
        let provider = YahooConnector::new();
        let reply: YResponse = provider.get_quote_history(ticker, start, end)?;
        let quotes: Vec<Quote> = reply.quotes()?;
        
        //println!("quotes: {:?}", quotes);
        Ok(PriceSeries{
            ticker: String::from(ticker),
            quotes,
            start,
            end 
        })
    }

    pub fn to_csv(& self) -> Vec<String> {
        // PriceSeries { 
        //     ticker: "MSFT", 
        //     quotes: [Quote { timestamp: 1640269800, open: 332.75, high: 336.3900146484375, 
        //                      low: 332.7300109863281, volume: 19611200, close: 334.69000244140625, 
        //                      adjclose: 334.69000244140625 }, 
        //              Quote { timestamp: 1640293204, open: 332.75, high: 336.3900146484375, 
        //                      low: 332.75, volume: 19617740, close: 334.69000244140625, 
        //                      adjclose: 334.69000244140625 }], 
        //     start: 2021-12-23T05:00:01Z, 
        //     end: 2021-12-25T18:10:25.244565Z 
        // }
        let period_start = &self.start.to_rfc3339();
        let ticker = &self.ticker;
        let quotes = &self.quotes;

        let end_quote = quotes
            .iter()
            .max_by(|q1, q2| q1.timestamp.cmp(&q2.timestamp))
            .unwrap();

        let start_quote = quotes
            .iter()
            .min_by(|q1, q2| q1.timestamp.cmp(&q2.timestamp))
            .unwrap();

        let min = quotes
            .iter()
            .min_by(|q1, q2| q1.low.partial_cmp(&q2.low).unwrap())
            .unwrap()
            .low;
            
        let max = quotes
            .iter()
            .max_by(|q1, q2| q1.high.partial_cmp(&q2.high).unwrap())
            .unwrap()
            .high;
        
        let sum: f64 = quotes 
            .iter()
            .rev()
            .take(30)
            .map(|q| q.adjclose)
            .sum();

        let count = quotes.iter().take(30).count();

        let average = sum / count as f64;

        let end_close = end_quote.adjclose;
        let start_close = start_quote.adjclose;

        let percent_change = 100.0 * (end_close - start_close) / start_close;

        println!("{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}", period_start, ticker, end_close, percent_change, min, max, average);
        //println!("max = {:?}\nmin = {:?}", end_quote, start_quote);
        //println!("start {}, ticker {}", period_start, ticker);
            
        // find min_by timestamp
        // find max_by timestamp
        let mut out = Vec::new();
        out.push(String::from("period_start,symbol,last_close_price,change_%,min,max,30d_avg"));
        println!("quotes: {:?}", self.quotes);
        out
    }
}

// cruft
//
//use yahoo_finance_api as yahoo;
    
// #[derive(Debug, Clone, Copy)]
// struct Quote {
//     timestamp: i64,
//     high: f32,
//     low: f32,
//     open: f32,
//     close: f32,
//     adjclose: f32,
//     volume: u64
// }

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
