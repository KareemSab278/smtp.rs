mod direct_smtp;
mod smtp;

const USE_DIRECT_TO_MX: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        if USE_DIRECT_TO_MX {
            println!("Sending email via direct_to_mx...");
            match direct_smtp::send_email("Test email", "This is a test email.").await {
                Ok(()) => println!("Email sent successfully."),
                Err(error) => eprintln!("Direct-to-MX email failed: {error}"),
            }
        } else {
            println!("Sending email via SMTP relay...");
            match smtp::send_email("Test email", "This is a test email.").await {
                Ok(()) => println!("Email sent successfully."),
                Err(error) => eprintln!("SMTP email failed: {error}"),
            }
        }
    });

    Ok(())
}

