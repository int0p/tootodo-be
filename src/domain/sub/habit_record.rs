use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HabitRecord {
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
    pub msg: String,
    pub photo: String,
}
