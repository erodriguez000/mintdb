use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct SQLRequest {
    pub stmt: String,
    pub tb: String,
    pub doc: String,
    pub data: Value,
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}
#[derive(Serialize, Debug)]
pub struct SQLResponse {
    status: String,
    message: String,
}