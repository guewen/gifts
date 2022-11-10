use lettre::smtp::authentication::IntoCredentials;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre::smtp::client::net::DEFAULT_TLS_PROTOCOLS;
use lettre_email::EmailBuilder;
use native_tls::TlsConnector;

use serde::{Deserialize, Serialize};

use pairs;

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
    pub fn format_body(&self, giver: &pairs::Person, receiver: &pairs::Person) -> String {
        self.body
            .replace("{giver}", &giver.name)
            .replace("{receiver}", &receiver.name)
    }
}

pub fn send_emails(server: &EmailServer, template: &EmailTemplate, pairs: &[pairs::Pair]) {
    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(DEFAULT_TLS_PROTOCOLS[0]));

    let tls_parameters =
        ClientTlsParameters::new(server.address.to_string(), tls_builder.build().unwrap());

    let creds = (&server.user, &server.password).into_credentials();
    let mut mailer = SmtpClient::new(
        (server.address.to_string(), server.port),
        ClientSecurity::Opportunistic(tls_parameters),
    ).unwrap()
        .credentials(creds)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    for pair in pairs.iter() {
        let body = template.format_body(&pair.giver, &pair.receiver);
        let email = EmailBuilder::new()
            .to(pair.giver.email.as_str())
            .from(template.from.as_str())
            .subject(&template.subject)
            .text(&body)
            .build()
            .unwrap()
            .into();
        // println!("{:?}", email);
        match mailer.send(email) {
            Ok(_) => println!("email successfully sent to {}", pair.giver.email),
            Err(err) => println!(
                "could not send email ({} -> {}): {}",
                pair.giver.email, pair.receiver.email, err
            ),
        }
    }

    mailer.close();
}
