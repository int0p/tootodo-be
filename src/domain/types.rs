use serde::{Deserialize, Serialize};

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
