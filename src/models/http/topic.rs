use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    pub topics: Vec<String>,
}