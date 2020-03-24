mod webhook_token;

use self::webhook_token::WebhookToken;
use cody_time;
use magic_8ball;
use rocket::response::status;
use rocket_contrib::json::Json;
use telegram_api;
use telegram_api::TelegramApiClient;
use token;

fn send_8_ball_response(chat_id: i64) {
    let message = magic_8ball::get_response();

    let client = TelegramApiClient::new(token::get());
    client.send_message(chat_id, message.to_owned());

    println!("Responded with: {:?}", message);
}

fn send_cody_time_response(chat_id: i64) {
    let message = cody_time::get_response();

    let client = TelegramApiClient::new(token::get());
    client.send_message(chat_id, message.to_owned());

    println!("Responsed with: {:?}", message);
}

#[post("/webhook/<_token>", data = "<update>")]
pub fn create(
    _token: WebhookToken,
    update: Json<telegram_api::types::Update>,
) -> status::Accepted<&'static str> {
    let deserialized_update = update.0;

    println!("{:?}", deserialized_update);

    match deserialized_update.message {
        Some(message) => {
            let message_text = message.text.unwrap_or("".to_owned());

            if message_text.contains("/8ball") {
                send_8_ball_response(message.chat.id);
            } else if message_text.contains("/codytime") {
                send_cody_time_response(message.chat.id);
            }
        }

        None => println!("Couldn't parse message"),
    }

    status::Accepted(Some("Accepted"))
}
