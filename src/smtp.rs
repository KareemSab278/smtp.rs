/*
    must be in ur .env file:
    SMTP_CONNECT_HOST - should be set to your SMTP server host, e.g., mail.mailerserver.co.uk
    SMTP_TLS_IDENTITY - should be set to your TLS identity, e.g., mail.extendcp.co.uk
    SMTP_PORT - should be set to your SMTP server port, e.g., 587
    SMTP_SECURITY - should be set to your SMTP security, e.g., starttls
    SMTP_TLS_MODE - should be set to your TLS mode, e.g., auto (can also be strict)
    SMTP_TLS_NAME - should be set to your TLS name, e.g., mail.mailerserver.co.uk
    SMTP_AUTH_METHOD - should be set to your authentication method, e.g., auto (can also be plain)
    SMTP_USERNAME - should be set to your SMTP username, e.g., alert@mailserver.co.uk
    SMTP_PASSWORD - should be set to your SMTP password, e.g., Volletom99
    MAIL_FROM - should be set to your sender email, e.g., alert@mailserver.co.uk
    MAIL_FROM_NAME - should be set to your sender name, e.g., test_sender
*/

// direct to mx direct_2mx


use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::time::Duration;

const RECIPIENT_EMAIL: &str = "kareem@coinadrink.co.uk";
const PRINT_CONFIG: bool = true;

#[allow(dead_code)]
struct Config {
    smtp_connect_host: &'static str,
    smtp_tls_identity: &'static str,
    smtp_port: u16,
    smtp_security: &'static str,
    smtp_tls_mode: &'static str,
    smtp_tls_name: &'static str,
    smtp_auth_method: &'static str,
    smtp_username: &'static str,
    smtp_password: &'static str,
    mail_from: &'static str,
    mail_from_name: &'static str,
}

fn print_config(config: &Config) -> () {
    if PRINT_CONFIG {
        println!("SMTP connect host: {}", config.smtp_connect_host);
        println!("SMTP port: {}", config.smtp_port);
        println!("SMTP security: {}", config.smtp_security);
        println!("SMTP TLS mode: {}", config.smtp_tls_mode);
        println!("SMTP TLS name: {}", config.smtp_tls_name);
        println!("SMTP auth method: {}", config.smtp_auth_method);
        println!("SMTP username: {}", config.smtp_username);
        println!("Mail from: {}", config.mail_from);
        println!("Mail from name: {}", config.mail_from_name);
        println!("Sending email to {}", RECIPIENT_EMAIL);
    }
}

fn required_env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|error| format!("{name}: {error}"))
}

fn get_env_vars() -> Result<Config, String> {
    // gets env variables and constructs a Config struct
    let smtp_connect_host = Box::leak(required_env("SMTP_CONNECT_HOST")?.into_boxed_str());
    let smtp_tls_identity = Box::leak(required_env("SMTP_TLS_IDENTITY")?.into_boxed_str());
    let smtp_port = required_env("SMTP_PORT")?
        .parse::<u16>()
        .map_err(|error| format!("SMTP_PORT: {error}"))?;
    let smtp_security = Box::leak(required_env("SMTP_SECURITY")?.into_boxed_str());
    let smtp_tls_mode = Box::leak(required_env("SMTP_TLS_MODE")?.into_boxed_str());
    let smtp_tls_name = Box::leak(required_env("SMTP_TLS_NAME")?.into_boxed_str());
    let smtp_auth_method = Box::leak(required_env("SMTP_AUTH_METHOD")?.into_boxed_str());
    let smtp_username = Box::leak(required_env("SMTP_USERNAME")?.into_boxed_str());
    let smtp_password = Box::leak(required_env("SMTP_PASSWORD")?.into_boxed_str());
    let mail_from = Box::leak(required_env("MAIL_FROM")?.into_boxed_str());
    let mail_from_name = Box::leak(required_env("MAIL_FROM_NAME")?.into_boxed_str());

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

    let sender_address = username.parse::<Address>().map_err(|e| e.to_string())?;

    let recipient_address = RECIPIENT_EMAIL
        .parse::<Address>()
        .map_err(|e| e.to_string())?;

    let email = Message::builder()
        .from(Mailbox::new(None, sender_address))
        .to(Mailbox::new(None, recipient_address))
        .subject(subject)
        .body(body.to_string())
        .map_err(|e| e.to_string())?;

    let credentials = Credentials::new(username.to_string(), password.to_string());

    let tls_identity = config.smtp_tls_identity.to_string();
    let tls = TlsParameters::new(tls_identity).map_err(|e| e.to_string())?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(config.smtp_connect_host)
        .port(config.smtp_port)
        .tls(Tls::Required(tls))
        .credentials(credentials)
        .timeout(Some(Duration::from_secs(15)))
        .build();

    let _ = print_config(&config);

    mailer
        .send(email)
        .await
        .map_err(|e| format!("Failed to send email: {e}"))?;

    Ok(())
}
