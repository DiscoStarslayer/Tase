use super::User;

#[derive(Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub tipe: String,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
}
