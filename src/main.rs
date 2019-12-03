extern crate lettre;
extern crate lettre_email;
extern crate rand;
extern crate serde;
extern crate serde_yaml;

use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

// use serde_yaml;

mod config;
mod email;
mod pairs;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config_file: config::ConfigFile = serde_yaml::from_str(&content).unwrap();
    println!("{:#?}", config_file);
    println!("Computing pairs...");
    let mut pool = pairs::Pool::new(config_file.people);
    let pairs = pool.make_pairs();
    println!("Pairs generated");

    println!("Sending emails");
    email::send_emails(&config_file.config.smtp, &config_file.config.email, &pairs);
    println!("Done!");

    // for pair in pairs.iter() {
    //     println!("{} → {}", pair.giver.email, pair.receiver.email);
    // }
    Ok(())
}
