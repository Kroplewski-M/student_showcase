use crate::errors::ErrorMessage;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct PostmarkEmail<'a> {
    #[serde(rename = "From")]
    from: &'a str,
    #[serde(rename = "To")]
    to: &'a str,
    #[serde(rename = "Subject")]
    subject: &'a str,
    #[serde(rename = "HtmlBody")]
    html_body: &'a str,
    #[serde(rename = "TextBody")]
    text_body: &'a str,
    #[serde(rename = "MessageStream")]
    message_stream: &'a str,
}

#[derive(Clone)]
pub struct EmailService {
    client: Client,
    from_email: String,
    server_token: String,
}
impl EmailService {
    pub async fn new() -> Self {
        let from_email =
            std::env::var("POSTMARK_FROM_EMAIL").expect("POSTMARK_FROM_EMAIL is missing");

        let server_token =
            std::env::var("POSTMARK_SERVER_TOKEN").expect("POSTMARK_SERVER_TOKEN is missing");

        let client = Client::new();

        Self {
            client,
            from_email,
            server_token,
        }
    }

    pub async fn send_email(
        &self,
        to_email: &str,
        subject: &str,
        text_part: &str,
        html_part: &str,
    ) -> Result<(), ErrorMessage> {
        let payload = PostmarkEmail {
            from: &self.from_email,
            to: to_email,
            subject,
            html_body: html_part,
            text_body: text_part,
            message_stream: "outbound",
        };

        let response = self
            .client
            .post("https://api.postmarkapp.com/email")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Postmark-Server-Token", &self.server_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?;

        if !response.status().is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(ErrorMessage::EmailSendingFailed(body));
        }

        Ok(())
    }
}
