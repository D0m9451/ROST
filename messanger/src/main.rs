use text_io::read;
use std::io::{self, Read, Write};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
        .from("dominic.j.milton@gmail.com".parse().unwrap())
        .to("japnesebean@gmail.com".parse().unwrap())
        .subject("Greetings")
        .body(String::from("stay attentive"))
        .unwrap();

    let creds = Credentials::new("19MiltonD@mysandstorm.org".to_string(), "".to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
}
