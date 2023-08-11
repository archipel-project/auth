use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T: Serialize> {
    pub code: u16,
    pub status: String,
    pub message: String,
    pub content: T
}