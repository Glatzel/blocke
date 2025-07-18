use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TabCoordSettings {
    pub projected_cs: String,
}
