use direct_to_mx::{Body, DirectToMx, DkimOptions};

const DEFAULT_RECIPIENT: &str = "kareem@coinadrink.co.uk";

pub async fn send_email(subject: &str, body: &str) -> Result<(), String> {
    let from_email = std::env::var("MAIL_FROM")
        .unwrap_or_else(|_| "alert@mailserver.co.uk".to_string());
    let from_name = std::env::var("MAIL_FROM_NAME").ok();
    let from = match from_name {
        Some(name) if !name.is_empty() => format!("{name} <{from_email}>"),
        _ => from_email,
    };

    let ehlo_hostname = std::env::var("EHLO_HOSTNAME")
        .or_else(|_| std::env::var("SMTP_TLS_NAME"))
        .or_else(|_| std::env::var("SMTP_CONNECT_HOST"))
        .unwrap_or_else(|_| "mailserver.co.uk".to_string());

    let recipient = std::env::var("MAIL_TO")
        .unwrap_or_else(|_| DEFAULT_RECIPIENT.to_string());

    let mut builder = DirectToMx::builder()
        .from(from)
        .ehlo_hostname(ehlo_hostname)
        .force_ipv4(true);

    // Optional DKIM configuration if provided via environment
    let dkim_selector = std::env::var("DKIM_SELECTOR").ok();
    let dkim_domain = std::env::var("DKIM_DOMAIN").ok();
    let dkim_key_path = std::env::var("DKIM_PRIVATE_KEY_PATH").ok();
    let dkim_key_pem = std::env::var("DKIM_PRIVATE_KEY_PEM").ok();

    if let (Some(selector), Some(domain)) = (dkim_selector, dkim_domain) {
        let pem = match (dkim_key_pem, dkim_key_path) {
            (Some(pem), _) => Some(pem),
            (None, Some(path)) => std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read DKIM key file '{path}': {e}"))
                .map(Some)?,
            (None, None) => None,
        };

        if let Some(private_key_pem) = pem {
            builder = builder.dkim(DkimOptions {
                selector,
                domain,
                private_key_pem,
            });
        }
    }

    let mailer = builder
        .build()
        .map_err(|e| format!("Failed to build DirectToMx mailer: {e}"))?;

    println!("Sending email via Direct-to-MX to {recipient}...");
    mailer
        .send(&recipient, subject, Body::text(body))
        .await
        .map_err(|e| format!("Direct-to-MX delivery failed: {e}"))?;

    Ok(())
}
