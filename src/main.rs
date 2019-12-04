extern crate clap;
extern crate lettre;
extern crate lettre_email;
extern crate rand;
extern crate serde;
extern crate serde_yaml;

use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

use clap::{App, Arg};

mod config;
mod email;
mod pairs;

fn main() -> io::Result<()> {
    let matches = App::new("Gifts!")
        .version("0.1.0")
        .author("Guewen Baconnier <guewen@gmail.com>")
        .about("Send Secret Emails for Secret-Santa-style events")
        .arg(
            Arg::with_name("config")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("Configuration File"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Do not send emails, show dummy results."),
        )
        .get_matches();

    let args: Vec<_> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config_file: config::ConfigFile = serde_yaml::from_str(&content).unwrap();
    println!("Computing pairs...");
    let mut pool = pairs::Pool::new(config_file.people);
    let pairs = pool.make_pairs();
    println!("Pairs generated");

    if matches.is_present("debug") {
        for pair in pairs.iter() {
            println!("{} → {}", pair.giver.email, pair.receiver.email);
        }
    } else {
        println!("Sending emails");
        email::send_emails(&config_file.config.smtp, &config_file.config.email, &pairs);
    }
    println!("Done!");

    Ok(())
}
