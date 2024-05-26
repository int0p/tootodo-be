pub mod req{
    use chrono::{DateTime, NaiveDate, Utc};
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateDailyTaskReq {
        pub task_id: ObjectId,
        pub title: String,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateDailyTaskReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub done: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub doneAt: Option<DateTime<Utc>>,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateDailyEventReq {
        pub event_id: ObjectId,
        pub title: String,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateDailyEventReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub done: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub doneAt: Option<DateTime<Utc>>,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateDailyHabitReq {
        pub habit_id: ObjectId,
        pub name: String,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateDailyHabitReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub done: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub doneAt: Option<DateTime<Utc>>,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateTimerResultReq {
        pub category_id: ObjectId,
        pub category_color: String,
        pub startAt: DateTime<Utc>,
        pub endAt: DateTime<Utc>,
        pub focus_time: String,
    }
    
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateTimerResultReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub startAt: Option<DateTime<Utc>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub endAt: Option<DateTime<Utc>>,
    }
    
}

pub mod res{

}