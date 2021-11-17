extern crate clap;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate tinytemplate;

use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

use clap::{App, AppSettings, Arg};

mod config;
mod email;
mod hints;
mod pairs;

fn generate_pairs_and_send_emails(config: config::ConfigFile, debug: bool) {
    println!("Computing pairs...");
    let mut nodes = vec![];
    let mut person_idx = 0;
    for (group_idx, group) in config.groups.iter().enumerate() {
        for person in group.people.iter() {
            nodes.push(pairs::Node::new(person_idx, group_idx, person.clone()));
            person_idx += 1;
        }
    }
    let mut pool = pairs::Pool::new(nodes);
    let pairs = pool.make_pairs().unwrap();
    println!("Pairs generated");

    let secret_hints = hints::Hints::new(pairs.clone());

    if debug {
        for pair in pairs.iter() {
            println!(
                "{} → {}",
                pair.giver.person.email, pair.receiver.person.email
            );
            let secrets = secret_hints.secret_hints(&pair.receiver);
            let body =
                config
                    .config
                    .email
                    .format_body(&pair.giver.person, &pair.receiver.person, secrets);
            println!(
                "{}",
                body.lines()
                    .map(|line| format!("  > {}\n", line))
                    .collect::<String>()
            )
        }
    } else {
        println!("Sending emails");
        email::send_emails(
            &config.config.smtp,
            &config.config.email,
            &pairs,
            &secret_hints,
        );
    }
    println!("Done!");
}

fn scaffold_config_and_create_file(output_path: &Path) -> io::Result<()> {
    let mut file = File::create(output_path)?;
    let message_body = "Hey {giver},\n\
                        \n\
                        The magical thingy decided that you'll \
                        offer a gift to... {receiver}.\n\n

                        {{- if has_secrets }}

                        I can tell you something...
                        {{ for secret in secrets }}
                        { secret.0.name } is the secret santa of { secret.1.name }... 🤫
                        {{- endfor }}
                        {{- endif }}";
    let config = config::ConfigFile::new(
        vec![
            config::Group::new(vec![
                config::Person::new("alice@example.com", "Alice"),
                config::Person::new("bob@example.com", "Bob"),
            ]),
            config::Group::new(vec![
                config::Person::new("jules@example.com", "Jules"),
                config::Person::new("janet@example.com", "Janet"),
            ]),
            config::Group::new(vec![config::Person::new("john@example.com", "John")]),
            config::Group::new(vec![
                config::Person::new("foo@example.com", "Foo"),
                config::Person::new("bar@example.com", "Bar"),
            ]),
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
        // TODO check duplicate people, maybe change structure
        file.read_to_string(&mut content)?;
        let config_file: config::ConfigFile = serde_yaml::from_str(&content).unwrap();
        generate_pairs_and_send_emails(config_file, matches.is_present("debug"));
    } else if let Some(output_path) = matches.value_of("scaffold") {
        scaffold_config_and_create_file(Path::new(output_path))?
    }

    Ok(())
}
