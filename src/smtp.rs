
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::time::Duration;

const RECIPIENT_EMAIL: &str = "kareem@coinadrink.co.uk";



/*
    must be in ur .env file:
    
    SMTP_CONNECT_HOST
    SMTP_TLS_IDENTITY
    SMTP_PORT
    SMTP_SECURITY
    SMTP_TLS_MODE
    SMTP_TLS_NAME
    SMTP_AUTH_METHOD
    SMTP_USERNAME
    SMTP_PASSWORD
    MAIL_FROM
    MAIL_FROM_NAME
*/
struct Config {
    smtp_connect_host: String,
    smtp_tls_identity: String,
    smtp_port: u16,
    smtp_security: String,
    smtp_tls_mode: String,
    smtp_tls_name: String,
    smtp_auth_method: String,
    smtp_username: String,
    smtp_password: String,
    mail_from: String,
    mail_from_name: String,
}

fn get_env_vars() -> Result<Config, String> {
    // gets env variables and constructs a Config struct
    let smtp_connect_host = std::env::var("SMTP_CONNECT_HOST").map_err(|e| e.to_string())?;
    let smtp_tls_identity = std::env::var("SMTP_TLS_IDENTITY").map_err(|e| e.to_string())?;
    let smtp_port = std::env::var("SMTP_PORT").map_err(|e| e.to_string())?.parse::<u16>().map_err(|e| e.to_string())?;
    let smtp_security = std::env::var("SMTP_SECURITY").map_err(|e| e.to_string())?;
    let smtp_tls_mode = std::env::var("SMTP_TLS_MODE").map_err(|e| e.to_string())?;
    let smtp_tls_name = std::env::var("SMTP_TLS_NAME").map_err(|e| e.to_string())?;
    let smtp_auth_method = std::env::var("SMTP_AUTH_METHOD").map_err(|e| e.to_string())?;
    let smtp_username = std::env::var("SMTP_USERNAME").map_err(|e| e.to_string())?;
    let smtp_password = std::env::var("SMTP_PASSWORD").map_err(|e| e.to_string())?;
    let mail_from = std::env::var("MAIL_FROM").map_err(|e| e.to_string())?;
    let mail_from_name = std::env::var("MAIL_FROM_NAME").map_err(|e| e.to_string())?;

    Ok(Config {
        smtp_connect_host,
        smtp_tls_identity,
        smtp_port,
        smtp_security,
        smtp_tls_mode,
        smtp_tls_name,
        smtp_auth_method,
        smtp_username,
        smtp_password,
        mail_from,
        mail_from_name,
    })
}


pub async fn send_email(subject: &str, body: &str) -> Result<(), String> {
    let config = get_env_vars()?;
    let username = config.smtp_username;
    let password = config.smtp_password;

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

    let tls = TlsParameters::new(config.smtp_tls_identity)
        .map_err(|e| e.to_string())?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(config.smtp_connect_host)
        .port(config.smtp_port)
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
