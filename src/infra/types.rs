use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

use crate::interface::dto::task::res::TaskRes;

// filter
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryFilterOptions {
    pub find_filter: Option<Document>,
    pub proj_opts: Option<Document>,
    pub limit: i64,
    pub page: i64,
}
// schedule
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ScheduleType {
    Task,
    Event,
    Habit,
}

// category, habit
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StatusType {
    InProgress,
    Archived,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PropertyType {
    MultiSelect,
    SingleSelect,

    Text,
    Number,
    DateTime,
    File,
    Image,
    Link,
    Email,
    Phone,
    Location,
}

//  chat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChatType {
    Ask,
    Event,
    Task,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MsgType {
    Text,
    Ask,
    Answer,
    Image,
    File,
    Link,
    Video,
    Audio,
    Location,
}

// task
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockType {
    Editor,
    Code,
    Drawing,
    Table,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PropValueType {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Serialize,  Debug, Clone)]
pub struct TaskTreeItem {
    pub task: TaskRes,
    pub subtasks: Vec<TaskTreeItem>, 
}