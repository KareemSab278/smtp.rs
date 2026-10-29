
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

// Hardcoded email configuration
const SENDER_EMAIL: &str = "";
const EMAIL_PASSWORD: &str = "";
const SMTP_SERVER: &str = "";
const SMTP_PORT: u16 = 465;
const RECIPIENT_EMAIL: &str = "recipient@example.com";

pub async fn send_email(subject: &str, body: &str) -> Result<(), String> {
    let sender_address = SENDER_EMAIL
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
        SENDER_EMAIL.to_string(),
        EMAIL_PASSWORD.to_string(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(SMTP_SERVER)
        .map_err(|e| e.to_string())?
        .port(SMTP_PORT)
        .credentials(credentials)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| format!("Failed to send email: {e}"))?;

    Ok(())
}
