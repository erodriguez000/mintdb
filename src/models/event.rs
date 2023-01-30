use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}