pub mod req {
    use serde::{Deserialize, Serialize};

    use crate::infra::types::StatusType;

    // Habit
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateHabitReq {
        pub name: String,
        pub icon: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateHabitReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<StatusType>,
    }
}

pub mod res {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::domain::habit::HabitModel;
    use crate::infra::types::StatusType;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug)]
    pub struct HabitRes {
        pub id: String,
        pub user: Uuid,
        pub name: String,
        pub icon: String,
        pub status: StatusType,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl HabitRes {
        pub fn from_model(habit: &HabitModel) -> Self {
            Self {
                id: habit.id.to_hex(),
                user: habit.user,
                name: habit.name.to_owned(),
                icon: habit.icon.to_owned(),
                status: habit.status.to_owned(),
                createdAt: habit.createdAt,
                updatedAt: habit.updatedAt,
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct HabitData {
        pub habit: HabitRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleHabitRes {
        pub status: &'static str,
        pub data: HabitData,
    }

    #[derive(Serialize, Debug)]
    pub struct HabitListRes {
        pub status: &'static str,
        pub results: usize,
        pub habits: Vec<HabitRes>,
    }
}
