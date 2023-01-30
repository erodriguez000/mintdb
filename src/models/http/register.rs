use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub user_id: usize,
}


#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub url: String,
}