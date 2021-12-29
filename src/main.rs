use chrono::{DateTime, Utc};
use basc::PriceSeries; 
use clap::{Arg, App, crate_authors, crate_version};
use std::process::exit;

fn main() {
    let matches = App::new("basc, a liveProject cli stock quote demonstrator")
        .version(&*format!("v{}", crate_version!()))
        .author(crate_authors!())
        .about("send stock pricing from Yahoo to stdout formatted as CSV")
        .arg(Arg::with_name("TICKER")
            .required(true)
            .multiple(true))
        .arg(Arg::with_name("START")
            .required(true)
            .takes_value(true)
            .short("s")
            .long("start")
            .help("starting date, as 2020-12-19"))
        .get_matches();

    // format, parse and convert DateTime<FixedOffset> -> DateTime<Utc>
    let start = format!("{}T00:00:01-05:00", matches.value_of("START").unwrap());
    let start: DateTime<Utc> = match DateTime::parse_from_rfc3339(&start) {
        Ok(start) => {
            start.into()
        },
        Err(_) => {
            eprintln!("Unable to parse date. For more information try --help");
            exit(1)
        }
    };
    let end: DateTime<Utc> = Utc::now();
    let tickers = matches.values_of("TICKER").unwrap();
            
    // write out the header
    println!("{}", PriceSeries::header());
            
    for ticker in tickers {
        if let Ok(series) = PriceSeries::from_range(ticker, start, end) {

            // write out ticker data
            println!("{}", series.to_csv());

            // demonstrate unused functions
            let vals = &series.to_prices();
            println!("{:?}", vals);
            println!("{:?}", basc::min(vals));
            println!("{:?}", basc::max(vals));
            println!("{:?}", basc::n_window_sma(2, vals));
            println!("{:?}", basc::price_diff(vals));

        } else {
            eprintln!("Unable to obtain quotes from Yahoo for {}, start {}, end {}",
                     ticker, start, end);
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