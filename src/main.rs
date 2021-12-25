use chrono::{DateTime, Utc};
use basc::PriceSeries; 


fn main() {

    let start: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-19T00:00:00-07:00").unwrap().into();
    let end: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-23T23:59:58-07:00").unwrap().into();
    
    
    if let Ok(series) = PriceSeries::from_range("AAPL", start, end) {
        println!("got {:?}", series);
    }
}
