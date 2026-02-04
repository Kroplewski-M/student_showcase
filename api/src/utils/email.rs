use crate::errors::ErrorMessage;
use aws_config::BehaviorVersion;
use aws_sdk_sesv2::{
    Client,
    types::{Body, Content, Destination, EmailContent, Message},
};

#[derive(Clone)]
pub struct EmailService {
    client: Client,
    from_email: String,
}

impl EmailService {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::v2026_01_12())
            .load()
            .await;
        let client = Client::new(&config);

        let from_email = std::env::var("SES_FROM_EMAIL").expect("SES_FROM_EMAIL IS MISSING");

        Self { client, from_email }
    }

    pub async fn send_email(
        &self,
        to_email: &str,
        subject: &str,
        text_part: &str,
        html_part: &str,
    ) -> Result<(), ErrorMessage> {
        let subject = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?;

        let body = Body::builder()
            .text(
                Content::builder()
                    .data(text_part)
                    .charset("UTF-8")
                    .build()
                    .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?,
            )
            .html(
                Content::builder()
                    .data(html_part)
                    .charset("UTF-8")
                    .build()
                    .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?,
            )
            .build();

        let message = Message::builder().subject(subject).body(body).build();

        let email = EmailContent::builder().simple(message).build();

        self.client
            .send_email()
            .from_email_address(&self.from_email)
            .destination(Destination::builder().to_addresses(to_email).build())
            .content(email)
            .send()
            .await
            .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?;

        Ok(())
    }
}
