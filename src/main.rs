use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use clap::{ArgGroup, Parser};

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
            nodes.push(pairs::Node::new(
                person_idx,
                group_idx,
                group.hints,
                person.clone(),
            ));
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
            config::Group::new(
                vec![
                    config::Person::new("alice@example.com", "Alice"),
                    config::Person::new("bob@example.com", "Bob"),
                ],
                true,
            ),
            config::Group::new(
                vec![
                    config::Person::new("jules@example.com", "Jules"),
                    config::Person::new("janet@example.com", "Janet"),
                ],
                false,
            ),
            config::Group::new(vec![config::Person::new("john@example.com", "John")], true),
            config::Group::new(
                vec![
                    config::Person::new("foo@example.com", "Foo"),
                    config::Person::new("bar@example.com", "Bar"),
                ],
                true,
            ),
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
    let content = serde_norway::to_string(&config).unwrap();
    fs::write(output_path, content)
}

/// Send Secret Emails for Secret-Santa-style events
#[derive(Parser, Debug)]
#[command(
    name = "Gifts!",
    version,
    author = "Guewen Baconnier <guewen@gmail.com>",
    arg_required_else_help = true,
    group(ArgGroup::new("action").args(["config", "scaffold"]))
)]
struct Cli {
    /// Configuration File
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Scaffold a configuration file
    #[arg(long, value_name = "FILE")]
    scaffold: Option<PathBuf>,

    /// Do not send emails, show dummy results.
    #[arg(short, long)]
    debug: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if let Some(config) = cli.config {
        // TODO check duplicate people, maybe change structure
        let content = fs::read_to_string(config)?;
        let config_file: config::ConfigFile = serde_norway::from_str(&content).unwrap();
        generate_pairs_and_send_emails(config_file, cli.debug);
    } else if let Some(output_path) = cli.scaffold {
        scaffold_config_and_create_file(&output_path)?
    }

    Ok(())
}
