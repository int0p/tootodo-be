use serde::Deserialize;

pub mod category;
pub mod daily;
pub mod event;
pub mod habit;
pub mod memo;
pub mod schedule;
pub mod task;
pub mod sub;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
