use std::env;

static TOKEN_KEY: &'static str = "TOKEN";

pub fn get() -> String {
    match env::var_os(TOKEN_KEY) {
        Some(val) => match val.to_str() {
            Some(val) => String::from(val),
            None => panic!("Could not decode TOKEN env string!"),
        },
        None => panic!("Please define the Telegram Bot token with the environment variable TOKEN"),
    }
}

pub fn is_valid(token: String) -> bool {
    token == get()
}
