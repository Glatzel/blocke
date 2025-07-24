use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TabCoordSettings {
    pub custom_cs: String,
}
