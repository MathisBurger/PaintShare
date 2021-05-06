use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: bool,
    pub message: String
}