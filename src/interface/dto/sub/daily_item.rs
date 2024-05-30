pub mod req {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateDailyTaskReq {
        pub task_id: String,
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
        pub event_id: String,
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
        pub habit_id: String,
        pub name: String,
        pub icon: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateDailyHabitReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub done: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub doneAt: Option<DateTime<Utc>>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateTimerResultReq {
        pub category_id: String,
        pub category_color: String,
        pub startAt: DateTime<Utc>,
        pub endAt: DateTime<Utc>,
        pub focus_time: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateTimerResultReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub startAt: Option<DateTime<Utc>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub endAt: Option<DateTime<Utc>>,
    }
}

pub mod res {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::domain::sub::daily_item::{
        DailyEventModel, DailyHabitModel, DailyTaskModel, TimerResultModel,
    };

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct DailyTaskRes {
        task_id: String,
        title: String,
        done: bool,
        doneAt: Option<DateTime<Utc>>,
    }
    impl DailyTaskRes {
        pub fn from_model(task: &DailyTaskModel) -> Self {
            Self {
                task_id: task.task_id.to_hex(),
                title: task.title.clone(),
                done: task.done,
                doneAt: task.doneAt,
            }
        }
    }
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct DailyEventRes {
        event_id: String,
        title: String,
        done: bool,
        doneAt: Option<DateTime<Utc>>,
    }
    impl DailyEventRes {
        pub fn from_model(event: &DailyEventModel) -> Self {
            Self {
                event_id: event.event_id.to_hex(),
                title: event.title.clone(),
                done: event.done,
                doneAt: event.doneAt,
            }
        }
    }
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct DailyHabitRes {
        habit_id: String,
        icon: String,
        name: String,
        done: bool,
        doneAt: Option<DateTime<Utc>>,
    }
    impl DailyHabitRes {
        pub fn from_model(habit: &DailyHabitModel) -> Self {
            Self {
                habit_id: habit.habit_id.to_hex(),
                icon: habit.icon.clone(),
                name: habit.name.clone(),
                done: habit.done,
                doneAt: habit.doneAt,
            }
        }
    }
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct TimerResultRes {
        category_id: String,
        category_color: String,
        startAt: DateTime<Utc>,
        endAt: DateTime<Utc>,
        focus_time: String,
    }
    impl TimerResultRes {
        pub fn from_model(timer_result: &TimerResultModel) -> Self {
            Self {
                category_id: timer_result.category_id.to_hex(),
                category_color: timer_result.category_color.clone(),
                startAt: timer_result.startAt,
                endAt: timer_result.endAt,
                focus_time: timer_result.focus_time.clone(),
            }
        }
    }
    #[derive(Serialize, Debug)]
    pub struct DailyItemData<T> {
        pub item: T,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleDailyItemRes<T> {
        pub status: &'static str,
        pub data: DailyItemData<T>,
    }

    #[derive(Serialize, Debug)]
    pub struct DailyItemListRes<T> {
        pub status: &'static str,
        pub results: usize,
        pub items: Vec<T>,
    }
}
