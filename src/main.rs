extern crate clap;
extern crate lettre;
extern crate lettre_email;
extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate native_tls;

use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use clap::{App, AppSettings, Arg};

mod config;
mod email;
mod pairs;

fn generate_pairs_and_send_emails(config: config::ConfigFile, debug: bool) {
    println!("Computing pairs...");
    let mut pool = pairs::Pool::new(config.people);
    let pairs = pool.make_pairs();
    println!("Pairs generated");

    if debug {
        for pair in pairs.iter() {
            println!("{} → {}", pair.giver.email, pair.receiver.email);
        }
    } else {
        println!("Sending emails");
        email::send_emails(&config.config.smtp, &config.config.email, &pairs);
    }
    println!("Done!");
}

fn scaffold_config_and_create_file(output_path: &Path) -> io::Result<()> {
    let mut file = File::create(output_path)?;
    let message_body = "Hey {giver},\n\
             \n\
             The magical thingy decided that you'll \
             offer a gift to... {receiver}.";
    let config = config::ConfigFile::new(
        vec![
            pairs::Person::new("alice@example.com", "Alice", Some(vec!["bob@example.com"])),
            pairs::Person::new("bob@example.com", "Bob", Some(vec!["alice@example.com"])),
            pairs::Person::new("jules@example.com", "Jules", Some(vec!["janet@example.com"])),
            pairs::Person::new("janet@example.com", "Janet", Some(vec!["jules@example.com"])),
        ],
        config::GeneralConfig::new(
            email::EmailServer::new("stmp.gmail.com", 587, "email-user@example.com", "password"),
            email::EmailTemplate::new(
                "email-user@example.com",
                "Gift for our Lackadaisical party",
                message_body,
            ),
        ),
    );
    let content = serde_yaml::to_string(&config).unwrap();
    file.write_all(content.as_bytes())
}

fn main() -> io::Result<()> {
    let matches = App::new("Gifts!")
        .version("0.1.0")
        .author("Guewen Baconnier <guewen@gmail.com>")
        .about("Send Secret Emails for Secret-Santa-style events")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Configuration File"),
        )
        .arg(
            Arg::with_name("scaffold")
                .long("scaffold")
                .takes_value(true)
                .help("Scaffold a configuration file"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Do not send emails, show dummy results."),
        )
        .get_matches();

    if let Some(config) = matches.value_of("config") {
        let mut file = File::open(config)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config_file: config::ConfigFile = serde_yaml::from_str(&content).unwrap();
        generate_pairs_and_send_emails(config_file, matches.is_present("debug"));
    } else if let Some(output_path) = matches.value_of("scaffold") {
        scaffold_config_and_create_file(Path::new(output_path))?
    }

    Ok(())
}
