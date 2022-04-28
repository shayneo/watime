use std::time::{SystemTime,UNIX_EPOCH};
use chrono::{DateTime, Utc};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value_t = String::from("m"))]
    fmt: String,
}


fn main() {
    let second_formats = [String::from("s"), String::from("second"), String::from("seconds")];
    let milli_formats = [String::from("m"), String::from("ms"), String::from("milli"), String::from("millis")];
    let nano_formats = [String::from("n"), String::from("ns"), String::from("nano"), String::from("nanos")];
    
    let args = Cli::parse();
    let time_fmt = args.fmt;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            if second_formats.contains(&time_fmt) {
                println!("{}", n.as_secs());
                return
            }
            if milli_formats.contains(&time_fmt) {
                println!("{}", n.as_millis());
                return
            }
            if nano_formats.contains(&time_fmt) {
                println!("{}", n.as_nanos());
                return
            }
            if time_fmt == "iso" {
                let now: DateTime<Utc> = Utc::now();
                println!("{}", now.to_rfc3339());
                return
            }
            if time_fmt == "wat" {
                println!("
                ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
                ██░███░█░▄▄▀█▄▄░▄▄█▄░█░
                ██░█░█░█░▀▀░███░███░▄█▄
                ██▄▀▄▀▄█░██░███░███▀██▀
                ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
            }
            if time_fmt != "" {
                let msg = format!("WAT? {} is an invalid format", time_fmt);
                panic!("{}", msg);
            }

            println!("{}", n.as_millis());
        },
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
