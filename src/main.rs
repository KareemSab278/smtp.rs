mod smtp;

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        println!("Sending email...");

        match smtp::send_email(
            "Test email",
            "This is a test email.",
        )
        .await
        {
            Ok(()) => println!("Email sent successfully."),
            Err(error) => eprintln!("Email failed: {error}"),
        }
    });
}