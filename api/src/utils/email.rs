use crate::{config::Config, errors::ErrorMessage, utils::generic};
use reqwest::Client;
use serde::Serialize;
use tera::{Context, Tera};
use tracing::error;
use uuid::Uuid;

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
    base_url: String,
    tera: Tera,
}
impl EmailService {
    pub async fn new(config: Config) -> Self {
        let client = Client::new();
        let tera = Tera::new("templates/**/*").expect("Failed to load email templates");

        Self {
            client,
            from_email: config.post_mark_config.mail_from_email.clone(),
            server_token: config.post_mark_config.server_token.clone(),
            base_url: config.base_url,
            tera,
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
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            error!(
                to = %to_email,
                status = %status.as_u16(),
                error_body = %body,
                "Postmark rejected auth email"
            );
            return Err(ErrorMessage::EmailSendingFailed(
                "An Error occured sending an email".to_string(),
            ));
        }

        Ok(())
    }
    pub async fn send_verification_email(
        &self,
        student_id: String,
        token: Uuid,
    ) -> Result<(), ErrorMessage> {
        let email = generic::get_email_for_student(student_id.as_str());
        let verify_url = format!("{}/verifytoken/{}", self.base_url, token);

        let mut ctx = Context::new();
        ctx.insert("verify_url", verify_url.as_str());
        let template = &self
            .tera
            .render("emails/verify_account.html", &ctx)
            .map_err(|e| ErrorMessage::EmailSendingFailed(e.to_string()))?;
        self.send_email(
            &email,
            "Verify Your Account",
            "Please verify your account using the link provided.",
            template,
        )
        .await
    }
}
