use serde::Deserialize;

pub mod category;
pub mod chat;
pub mod daily;
pub mod event;
pub mod habit;
pub mod memo;
pub mod schedule;
pub mod task;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
