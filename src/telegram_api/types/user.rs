#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}
