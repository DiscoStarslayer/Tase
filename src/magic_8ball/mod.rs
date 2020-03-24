mod answers;

use rand;
use rand::Rng;

pub fn get_response() -> &'static str {
    rand::thread_rng()
        .choose(&answers::MAGIC_8BALL_ANSWERS)
        .unwrap_or(&"Try Again Later")
}
