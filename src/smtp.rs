
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::time::Duration;

const SMTP_CONNECT_HOST: &str = "mail.mailerserver.co.uk";
// if host cert failed use this identity for TLS verification
const SMTP_TLS_IDENTITY: &str = "mail.extendcp.co.uk";

const SMTP_PORT: u16 = 587;
const RECIPIENT_EMAIL: &str = "kareem@coinadrink.co.uk";

pub async fn send_email(subject: &str, body: &str) -> Result<(), String> {
    let username = "fridge@mailserver.co.uk".to_string();
    let password = "Volletom99".to_string();

    let sender_address = username
        .parse::<Address>()
        .map_err(|e| e.to_string())?;

    let recipient_address = RECIPIENT_EMAIL
        .parse::<Address>()
        .map_err(|e| e.to_string())?;

    let email = Message::builder()
        .from(Mailbox::new(None, sender_address))
        .to(Mailbox::new(None, recipient_address))
        .subject(subject)
        .body(body.to_string())
        .map_err(|e| e.to_string())?;

    let credentials = Credentials::new(
        username,
        password,
    );

    let tls = TlsParameters::new(SMTP_TLS_IDENTITY.to_owned())
        .map_err(|e| e.to_string())?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(SMTP_CONNECT_HOST)
        .port(SMTP_PORT)
        .tls(Tls::Required(tls))
        .credentials(credentials)
        .timeout(Some(Duration::from_secs(15)))
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| format!("Failed to send email: {e}"))?;

    Ok(())
}
