pub mod req {
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc, Weekday};
    use serde::{Deserialize, Serialize};

    use crate::infra::types::ScheduleType;

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateScheduledTaskReq {
        pub task_id: String,
        pub title: String,
        pub category_id: String,
        pub category_color: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateScheduledTaskReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_color: Option<String>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateScheduledEventReq {
        pub event_id: String,
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
        pub habit_id: String,
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
        pub item_id: String,
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

pub mod res {
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc, Weekday};
    use serde::{Deserialize, Serialize};

    use crate::domain::sub::schedule_item::{
        ScheduledAt, ScheduledEvent, ScheduledHabit, ScheduledTask,
    };
    use crate::infra::types::ScheduleType;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ScheduledTaskRes {
        task_id: String,
        title: String,
        category_id: String,
        category_color: String,
    }

    impl ScheduledTaskRes {
        pub fn from_model(task: &ScheduledTask) -> Self {
            Self {
                task_id: task.task_id.to_hex(),
                title: task.title.clone(),
                category_id: task.category_id.to_hex(),
                category_color: task.category_color.clone(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ScheduledEventRes {
        event_id: String,
        title: String,
        start_date: Option<NaiveDate>,
        due_at: Option<DateTime<Utc>>,
    }

    impl ScheduledEventRes {
        pub fn from_model(event: &ScheduledEvent) -> Self {
            Self {
                event_id: event.event_id.to_hex(),
                title: event.title.clone(),
                start_date: event.start_date,
                due_at: event.due_at,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ScheduledHabitRes {
        habit_id: String,
        name: String,
        icon: String,
    }

    impl ScheduledHabitRes {
        pub fn from_model(habit: &ScheduledHabit) -> Self {
            Self {
                habit_id: habit.habit_id.to_hex(),
                name: habit.name.clone(),
                icon: habit.icon.clone(),
            }
        }
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ScheduledAtRes {
        item_id: String,
        item_type: ScheduleType,
        weekday: Weekday,
        startAt: Option<NaiveTime>,
        endAt: Option<NaiveTime>,
    }

    impl ScheduledAtRes {
        pub fn from_model(scheduled_at: &ScheduledAt) -> Self {
            Self {
                item_id: scheduled_at.item_id.to_hex(),
                item_type: scheduled_at.item_type.clone(),
                weekday: scheduled_at.weekday,
                startAt: scheduled_at.startAt,
                endAt: scheduled_at.endAt,
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct ScheduleItemData<T> {
        pub item: T,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleScheduleItemRes<T> {
        pub status: &'static str,
        pub data: ScheduleItemData<T>,
    }

    #[derive(Serialize, Debug)]
    pub struct ScheduleItemListRes<T> {
        pub status: &'static str,
        pub results: usize,
        pub items: Vec<T>,
    }
}
