#[derive(Deserialize, Debug)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f32,
    pub y_shift: f32,
    pub scale: f32,
}
