use lettre::smtp::authentication::IntoCredentials;
use lettre::smtp::client::net::DEFAULT_TLS_PROTOCOLS;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use native_tls::TlsConnector;

use serde::{Deserialize, Serialize};

use tinytemplate::TinyTemplate;

use config;
use hints;
use pairs;

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
    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(DEFAULT_TLS_PROTOCOLS[0]));

    let tls_parameters =
        ClientTlsParameters::new(server.address.to_string(), tls_builder.build().unwrap());

    let creds = (&server.user, &server.password).into_credentials();
    let mut mailer = SmtpClient::new(
        (server.address.to_string(), server.port),
        ClientSecurity::Opportunistic(tls_parameters),
    )
    .unwrap()
    .credentials(creds)
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    for pair in pairs.iter() {
        let secrets = hints.secret_hints(&pair.receiver);
        let body = template.format_body(&pair.giver.person, &pair.receiver.person, secrets);
        let email = EmailBuilder::new()
            .to(pair.giver.person.email.as_str())
            .from(template.from.as_str())
            .subject(&template.subject)
            .text(&body)
            .build()
            .unwrap()
            .into();

        match mailer.send(email) {
            Ok(_) => println!("email successfully sent to {}", pair.giver.person.email),
            Err(err) => println!(
                "could not send email ({} -> {}): {}",
                pair.giver.person.email, pair.receiver.person.email, err
            ),
        }
    }

    mailer.close();
}
