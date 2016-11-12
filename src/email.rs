use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::EmailTransport;

use pairs;

#[derive(Debug)]
pub struct EmailServer<'a> {
    host: &'a str,
    port: u16,
    user: &'a str,
    password: &'a str,
}

impl<'a> EmailServer<'a> {
    pub fn new(host: &'a str, port: u16, user: &'a str, password: &'a str) -> EmailServer<'a> {
        EmailServer {
            host: host,
            port: port,
            user: user,
            password: password,
        }
    }
}


#[derive(Debug)]
pub struct EmailTemplate<'a> {
    from: &'a str,
    subject: &'a str,
    body: &'a str,
}

impl<'a> EmailTemplate<'a> {
    pub fn new(from: &'a str, subject: &'a str, body: &'a str) -> EmailTemplate<'a> {
        EmailTemplate {
            from: from,
            subject: subject,
            body: body,
        }
    }

    pub fn format_body(&self, giver: &pairs::Person, receiver: &pairs::Person) -> String{
        self.body.replace("{giver}", giver.name)
            .replace("{receiver}", receiver.name)
    }
}


pub fn send_emails(server: &EmailServer, template: &EmailTemplate, pairs: &Vec<pairs::Pair>) {
    let mut mailer = SmtpTransportBuilder::new(
            (server.host, server.port))
        .unwrap()
        // Set the name sent during EHLO/HELO, default is `localhost`
        // .hello_name("localhost")
        // Add credentials for authentication
        .credentials(server.user, server.password)
        // Specify a TLS security level. You can also specify an SslContext with
        // .ssl_context(SslContext::Ssl23)
        .security_level(SecurityLevel::AlwaysEncrypt)
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Enable connection reuse
        .connection_reuse(true).build();

    for pair in pairs.iter() {
        let email = EmailBuilder::new()
                            .to(pair.giver.email)
                            .from(template.from)
                            .body(&template.format_body(&pair.giver, &pair.receiver))
                            .subject(template.subject)
                            .build()
                            .unwrap();
        // println!("{:?}", email);
        match mailer.send(email) {
            Ok(_) => println!("email successfully sent to {}", pair.giver.email),
            Err(err) => println!("could not send email ({} -> {}): {}", pair.giver.email, pair.receiver.email, err)
        }
    }

    mailer.close();
}
