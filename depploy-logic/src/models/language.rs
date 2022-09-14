use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Language{
    name: String,
    usage: String,
    extensions: Vec<String>,
}