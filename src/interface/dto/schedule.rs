pub mod req {
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc, Weekday};
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    use crate::domain::types::ScheduleType;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateScheduledTaskReq {
        pub task_id: ObjectId,
        pub title: String,
        pub category_id: ObjectId,
        pub category_color: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateScheduledTaskReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_color: Option<String>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateScheduledEventReq {
        pub event_id: ObjectId,
        pub title: String,
        pub start_date: Option<NaiveDate>,
        pub due_at: Option<DateTime<Utc>>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateScheduledEventReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Utc>>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateScheduledHabitReq {
        pub habit_id: ObjectId,
        pub name: String,
        pub icon: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateScheduledHabitReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct CreateScheduledAtReq {
        pub item_id: ObjectId,
        pub item_type: ScheduleType,
        pub weekday: Weekday, //Mon=0
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct UpdateScheduledAtReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub item_type: Option<ScheduleType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub weekday: Option<Weekday>, //Mon=0
        #[serde(skip_serializing_if = "Option::is_none")]
        pub startAt: Option<NaiveTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub endAt: Option<NaiveTime>,
    }
}

pub mod res {}
