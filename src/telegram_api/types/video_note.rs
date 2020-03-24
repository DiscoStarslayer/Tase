use super::PhotoSize;

#[derive(Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub length: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i32>,
}
