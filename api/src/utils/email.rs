use reqwest::Client;
use serde::Serialize;

use crate::errors::ErrorMessage;

#[derive(Serialize)]
struct Message {
    to: String,
    from: String,
    subject: String,
    text_part: String,
    html_part: String,
}

#[derive(Clone)]
pub struct EmailService {
    client: Client,
    from_email: String,
    api_key_public: String,
    api_key_private: String,
}

impl EmailService {
    pub fn new() -> Self {
        let private_key = std::env::var("MAILJET_API_SECRET").expect("MAILJET API KEY IS MISSING");
        let public_key = std::env::var("MAILJET_API_KEY").expect("MAILJET API SECRET IS MISSING");
        let from_email =
            std::env::var("MAILJET_API_FROM_EMAIL").expect("MAILJET API FROM EMAIL IS MISSING");

        Self {
            client: Client::new(),
            api_key_public: public_key,
            api_key_private: private_key,
            from_email,
        }
    }
    pub async fn send_email(
        &self,
        to_email: &str,
        subject: &str,
        text_part: &str,
        html_part: &str,
    ) -> Result<(), ErrorMessage> {
        let payload = Message {
            to: to_email.to_string(),
            from: self.from_email.to_string(),
            subject: subject.to_string(),
            text_part: text_part.to_string(),
            html_part: html_part.to_string(),
        };

        let response = self
            .client
            .post("https://api.mailjet.com/v3.1/send")
            .basic_auth(&self.api_key_public, Some(self.api_key_private.clone()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?;

        if !response.status().is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Failed to read response body: {e}"));

            return Err(ErrorMessage::EmailSendingFailed(body));
        }
        Ok(())
    }
}
