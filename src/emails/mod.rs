use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre::smtp::authentication::Credentials;
use lettre_email::EmailBuilder;
use native_tls::{Protocol, TlsConnector};

use crate::configuration::*;

pub fn send_email(to: &str, from: &str, subject: &str, body: &str) {
    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("Smtp Host:{}", &configuration.email.host);
    let email = EmailBuilder::new()
        .from(from)
        .to(to)
        .subject(subject)
        .text(String::from(body))
        .build()
        .unwrap();

    let creds = Credentials::new(
        configuration.email.username.to_string(),
        configuration.email.password.to_string(),
    );
    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Sslv3));
    tls_builder.use_sni(false);
    tls_builder.danger_accept_invalid_certs(true);
    tls_builder.danger_accept_invalid_hostnames(true);
    let tls_parameters = ClientTlsParameters::new(
        configuration.email.host.to_string(),
        tls_builder.build().unwrap(),
    );
    // Open a remote connection to email
    let mut mailer = SmtpClient::new(
        (
            configuration.email.host.to_string(),
            configuration.email.port,
        ),
        ClientSecurity::Required(tls_parameters),
    )
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    match mailer.send(email.into()) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
