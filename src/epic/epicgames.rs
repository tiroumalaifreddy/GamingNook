use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EpicGame {
    pub product_id: String,
    pub sandbox_name: String
}

