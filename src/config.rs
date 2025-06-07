use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub zone_id: String,
    pub record: String,
    pub ttl: i64,
}
