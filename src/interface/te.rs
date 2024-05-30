#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TaskFetchRes {
    pub id: String,
    pub user: Uuid,
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub due_at: Option<DateTime<Utc>>,

    pub category_id: String,
    pub prop_values: Vec<PropValueModel>,

    pub subtasks: Vec<TaskFetchRes>,

    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

impl TaskFetchRes {
    pub fn from_model(task: &TaskModel) -> Self {
        Self {
            id: task.id.to_hex(),
            user: task.user,
            title: task.title.to_owned(),
            start_date: task.start_date.to_owned(),
            due_at: task.due_at.to_owned(),
            category_id: task.category_id.to_hex(),
            prop_values: task.prop_values.clone(),
            subtasks: task.subtasks.to_owned(),
            createdAt: task.createdAt,
            updatedAt: task.updatedAt,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TaskFetchRes {
    pub status: &'static str,
    pub results: usize,
    pub tasks: Vec<TaskFetchRes>,
}
