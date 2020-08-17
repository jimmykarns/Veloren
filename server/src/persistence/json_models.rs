use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub species: i16,
    pub body_type: i16,
    pub hair_style: i16,
    pub beard: i16,
    pub eyes: i16,
    pub accessory: i16,
    pub hair_color: i16,
    pub skin: i16,
    pub eye_color: i16,
}
