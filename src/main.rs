extern crate rand;
extern crate lettre;
extern crate yaml_rust;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

use yaml_rust::yaml;

mod email;
mod pairs;


fn main() {
    let args: Vec<_> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let docs = yaml::YamlLoader::load_from_str(&content).unwrap();
    let doc = &docs[0];
    let people = match doc["people"] {
        yaml::Yaml::Array(ref r) => r,
        yaml::Yaml::BadValue => panic!("No people in the yaml file"),
        _ => panic!("Wrong format of people")
    };
    let mut all_people = vec!();
    println!("Computing pairs...");
    for person in people.iter() {
        let name = match person["name"] {
            yaml::Yaml::String(ref r) => r,
            yaml::Yaml::BadValue => panic!("Missing 'name' for a person {:?}", person),
            _ => panic!("Wrong format of 'name' for a person {:?}", person)
        };
        let email = match person["email"] {
            yaml::Yaml::String(ref r) => r,
            yaml::Yaml::BadValue => panic!("Missing 'email' for a person {:?}", person),
            _ => panic!("Wrong format of 'email' for a person {:?}", person)
        };
        let exclude = match person["exclude"] {
            yaml::Yaml::Array(ref r) => r.iter().map(|s| s.as_str().unwrap()).collect(),
            yaml::Yaml::Null => vec!(),
            yaml::Yaml::BadValue => vec!(),
            _ => panic!("Wrong format of 'exclude' for a person {:?}", person)
        };
        let person = pairs::Person::new(email, name, exclude);
        all_people.push(person);
    }
    let mut pool = pairs::Pool::new(all_people.iter().collect::<Vec<&pairs::Person>>());
    let pairs = pool.make_pairs();
    println!("Pairs generated");


    let config: BTreeMap<&str, &yaml_rust::Yaml> = match doc["config"] {
        yaml::Yaml::Hash(ref r) => r.iter().map(|(k, v)| (k.as_str().unwrap(), v)).collect(),
        yaml::Yaml::BadValue => panic!("No 'config' in the yaml file"),
        _ => panic!("Wrong format of 'config'")
    };

    let smtp_config: BTreeMap<&str, &yaml_rust::Yaml> = match config["smtp"] {
        &yaml::Yaml::Hash(ref r) => r.iter().map(|(k, v)| (k.as_str().unwrap(), v)).collect(),
        &yaml::Yaml::BadValue => panic!("No 'config→smtp' in the yaml file"),
        _ => panic!("Wrong format of 'config→smtp'")
    };
    let host = match smtp_config["host"] {
        &yaml::Yaml::String(ref r) => r,
        _ => panic!("Wrong format or missing 'config→smtp→host'")
    };
    let port = match smtp_config["port"] {
        &yaml::Yaml::Integer(ref r) => r,
        _ => panic!("Wrong format or missing 'config→smtp→port'")
    };
    let user = match smtp_config["user"] {
        &yaml::Yaml::String(ref r) => r,
        _ => panic!("Wrong format or missing 'config→smtp→user'")
    };
    let password = match smtp_config["password"] {
        &yaml::Yaml::String(ref r) => r,
        _ => panic!("Wrong format or missing 'config→smtp→password'")
    };

    let email_server = email::EmailServer::new(host, *port as u16, user, password);

    let email_config: BTreeMap<&str, &yaml_rust::Yaml> = match config["email"] {
        &yaml::Yaml::Hash(ref r) => r.iter().map(|(k, v)| (k.as_str().unwrap(), v)).collect(),
        &yaml::Yaml::BadValue => panic!("No 'config→email' in the yaml file"),
        _ => panic!("Wrong format of 'config→email'")
    };
    let from = match email_config["from"] {
        &yaml::Yaml::String(ref r) => r,
        _ => panic!("Wrong format or missing 'config→email→from'")
    };
    let subject = match email_config["subject"] {
        &yaml::Yaml::String(ref r) => r,
        _ => panic!("Wrong format or missing 'config→email→subject'")
    };
    let body = match email_config["body"] {
        &yaml::Yaml::String(ref r) => r,
        _ => panic!("Wrong format or missing 'config→email→body'")
    };
    let email_template = email::EmailTemplate::new(from, subject, body);

    println!("Sending emails");
    email::send_emails(&email_server, &email_template, &pairs);
    println!("Done!");

    // for pair in pairs.iter() {
    //     println!("{} → {}", pair.giver.email, pair.receiver.email);
    // }

}
