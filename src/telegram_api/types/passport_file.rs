#[derive(Deserialize, Debug)]
pub struct PassportFile {
    pub file_id: String,
    pub file_size: i32,
    pub file_date: i32,
}
