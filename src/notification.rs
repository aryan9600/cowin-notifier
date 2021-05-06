use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub center_name: String,
    pub address: String,
    pub date: String,
}
