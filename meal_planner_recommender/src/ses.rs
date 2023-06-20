use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_ses::{
    types::{Body, Content, Destination, Message},
    Client,
};

const CHARSET: &str = "UTF-8";
const SUBJECT: &str = "Meal Planner";

pub struct Ses {
    client: Client,
}

impl Ses {
    pub async fn init(config: &SdkConfig) -> Self {
        Self {
            client: Client::new(config),
        }
    }

    pub async fn send_email(&self, body_content: String) -> Result<()> {
        let subject = Content::builder().charset(CHARSET).data(SUBJECT).build();
        let html_body = Content::builder()
            .charset(CHARSET)
            .data(body_content)
            .build();
        let body = Body::builder().html(html_body).build();
        let message = Message::builder().body(body).subject(subject).build();
        let destination = Destination::builder()
            .to_addresses("bbstilson@fastmail.com")
            .build();
        self.client
            .send_email()
            .source("bbstilson@fastmail.com")
            .destination(destination)
            .message(message)
            .reply_to_addresses("bbstilson@fastmail.com")
            .send()
            .await?;

        Ok(())
    }
}
