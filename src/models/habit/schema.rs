use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::StatusType;

// Habit
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHabitSchema {
    pub name: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHabitSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusType>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
