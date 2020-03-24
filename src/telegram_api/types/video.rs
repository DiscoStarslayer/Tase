use super::PhotoSize;

#[derive(Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}
