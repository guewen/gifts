use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};

use serde::{Deserialize, Serialize};

use tinytemplate::TinyTemplate;

use crate::config;
use crate::hints;
use crate::pairs;

#[derive(Serialize)]
struct EmailBodyContext {
    giver: String,
    receiver: String,
    has_secrets: bool,
    secrets: Vec<(config::Person, config::Person)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailServer {
    address: String,
    port: u16,
    user: String,
    password: String,
}

impl EmailServer {
    pub fn new(address: &str, port: u16, user: &str, password: &str) -> EmailServer {
        EmailServer {
            address: address.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    from: String,
    subject: String,
    body: String,
}

impl EmailTemplate {
    pub fn new(from: &str, subject: &str, body: &str) -> EmailTemplate {
        EmailTemplate {
            from: from.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        }
    }
    pub fn format_body(
        &self,
        giver_person: &config::Person,
        receiver_person: &config::Person,
        secrets: Vec<(config::Person, config::Person)>,
    ) -> String {
        let mut template = TinyTemplate::new();
        template.add_template("body", &self.body).unwrap();

        let context = EmailBodyContext {
            giver: giver_person.name.clone(),
            receiver: receiver_person.name.clone(),
            has_secrets: !secrets.is_empty(),
            secrets,
        };

        template.render("body", &context).unwrap()
    }
}

pub fn send_emails(
    server: &EmailServer,
    template: &EmailTemplate,
    pairs: &[pairs::Pair],
    hints: &hints::Hints,
) {
    let tls_parameters = TlsParameters::new(server.address.clone()).unwrap();
    let creds = Credentials::new(server.user.clone(), server.password.clone());
    let mailer = SmtpTransport::builder_dangerous(&server.address)
        .port(server.port)
        .tls(Tls::Opportunistic(tls_parameters))
        .credentials(creds)
        .build();

    for pair in pairs.iter() {
        let secrets = hints.secret_hints(&pair.receiver);
        let body = template.format_body(&pair.giver.person, &pair.receiver.person, secrets);
        let email = match build_message(template, &pair.giver.person.email, body) {
            Ok(email) => email,
            Err(err) => {
                println!(
                    "could not build email ({} -> {}): {}",
                    pair.giver.person.email, pair.receiver.person.email, err
                );
                continue;
            }
        };

        match mailer.send(&email) {
            Ok(_) => println!("email successfully sent to {}", pair.giver.person.email),
            Err(err) => println!(
                "could not send email ({} -> {}): {}",
                pair.giver.person.email, pair.receiver.person.email, err
            ),
        }
    }
}

fn build_message(
    template: &EmailTemplate,
    to: &str,
    body: String,
) -> Result<Message, Box<dyn std::error::Error>> {
    Ok(Message::builder()
        .to(to.parse()?)
        .from(template.from.parse()?)
        .subject(&template.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?)
}
