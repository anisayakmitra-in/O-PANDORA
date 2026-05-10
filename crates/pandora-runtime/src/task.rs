#[derive(
    Debug,
    Clone,
)]
pub enum TaskStatus {

    Pending,

    Running,

    Completed,

    Failed,
}

#[derive(
    Debug,
    Clone,
)]
pub struct RuntimeTask {

    pub id: String,

    pub task_type: String,

    pub target: String,

    pub status: TaskStatus,
}
