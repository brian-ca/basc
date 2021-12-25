use chrono::{DateTime, Utc};
use basc::PriceSeries; 


fn main() {

    let start: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-19T16:39:57-08:00").unwrap().into();
    let end: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-19T16:39:57-08:00").unwrap().into();
    
    
    if let Ok(series) = PriceSeries::new("hi", start, end) {
        println!("got {:?}", series);
    }
}
