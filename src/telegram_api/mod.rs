pub mod types;

use reqwest::{Client, Url};
use std::time::Duration;

pub struct TelegramApiClient {
    client: Client,
    token: String,
}

impl TelegramApiClient {
    pub fn new(token: String) -> TelegramApiClient {
        TelegramApiClient {
            client: Client::builder()
                .gzip(true)
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            token: token,
        }
    }

    pub fn send_message(&self, chat_id: i64, message: String) {
        let uri = self.create_uri(String::from("sendMessage"));
        let payload = build_markdown_message(chat_id, message);

        println!("{:?}", uri);
        println!("{:?}", payload);

        let send_result = &self.client.post(uri).json(&payload).send();

        match send_result {
            Ok(response) => {
                println!("POST sendMessage status: {}", response.status());
            }
            Err(err) => {
                println!("POST sendMessage failed: {:?}", err);
            }
        }
    }

    fn create_uri(&self, method_name: String) -> Url {
        let url_string = format!(
            "https://api.telegram.org/bot{token}/{method_name}",
            token = &self.token,
            method_name = method_name
        );

        Url::parse(&url_string).unwrap()
    }
}

fn build_markdown_message(chat_id: i64, text: String) -> types::SendMessage {
    types::SendMessage {
        chat_id: chat_id,
        text: text,
        parse_mode: Some(String::from("Markdown")),
    }
}
