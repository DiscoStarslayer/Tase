use super::PhotoSize;

#[derive(Deserialize, Debug)]
pub struct Animation {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}
