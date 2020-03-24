use super::PhotoSize;

#[derive(Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
    pub thumb: Option<PhotoSize>,
}
