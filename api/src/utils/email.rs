use crate::errors::ErrorMessage;
use reqwest::Client;
use serde::Serialize;

/// Represents the JSON payload expected by the Postmark `/email` API.
/// Field names must match Postmark's casing exactly, hence the serde renames.
#[derive(Serialize)]
struct PostmarkEmail<'a> {
    /// Sender email address (must be verified in Postmark)
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

/// Email service responsible for sending emails via Postmark
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
    /// Sends an email using Postmark.
    ///
    /// - `to_email`   → recipient address
    /// - `subject`    → email subject
    /// - `text_part`  → plain-text body
    /// - `html_part`  → HTML body
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
