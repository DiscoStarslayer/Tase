use rocket::http::RawStr;
use rocket::request::FromParam;
use token;

pub struct WebhookToken(bool);

impl<'r> FromParam<'r> for WebhookToken {
    type Error = &'static str;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let decoded_param = param.percent_decode();

        match decoded_param {
            Ok(param) => {
                if token::is_valid(param.into()) {
                    return Ok(WebhookToken(true));
                }

                Err("Invalid Token")
            }
            Err(_) => Err("Invalid Token"),
        }
    }
}
