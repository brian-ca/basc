use chrono::{DateTime, Utc};
use basc::PriceSeries; 
use clap::{Arg, App, crate_authors, crate_version};

fn main() {
    let matches = App::new("basc, a liveProject cli stock quote demonstrator")
        .version(&*format!("v{}", crate_version!()))
        .author(crate_authors!())
        .about("send stock pricing from Yahoo to stdout formatted as CSV")
        .arg(Arg::with_name("TICKER")
            .required(true))
        .arg(Arg::with_name("START")
            .required(true)
            .takes_value(true)
            .short("s")
            .long("start")
            .help("starting date, as 2020-12-19"))
        .get_matches();

    // String
    let start = format!("{}T00:00:01-05:00", matches.value_of("START").unwrap());
    let ticker = matches.value_of("TICKER").unwrap();

    // DateTime<FixedOffset>
    match DateTime::parse_from_rfc3339(&start) {
        Ok(start) => {
            // DateTime<Utc>
            let start: DateTime<Utc> = start.into();
            let end: DateTime<Utc> = Utc::now();
            //println!("Start {}, End {}", start, end);

            if let Ok(series) = PriceSeries::from_range(ticker, start, end) {
                println!("PriceSeries retrieved: {:?}", series);
            } else {
                eprintln!("Unable to obtain quotes from Yahoo for {}, start {}, end {}",
                        ticker, start, end);
            }
        },
        Err(_) => {
            eprintln!("Unable to parse date. For more information try --help")
        }
    }
}

// notes:
//
// . only daily times supported.  uncertain of yahoo time base, so using UTC-5 midnight + 1 and
// . UTC-5 midnight -1 for day start and end for now.  NYC is Eastern.  EST is UTC-5.
//


// cruft
//
    //let start: DateTime<Utc> = DateTime::parse_from_rfc339(format!("{}T00:00:01-00:00", matches.))
    //let start: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-19T00:00:00-07:00").unwrap().into();
    //let end: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-23T23:59:58-07:00").unwrap().into();
    
    
    //if let Ok(series) = PriceSeries::from_range("AAPL", start, end) {
    //    println!("got {:?}", series);
    //}