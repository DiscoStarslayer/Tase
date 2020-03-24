#[derive(Serialize, Debug)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
}
