use crate::task::{
    RuntimeTask,
    TaskStatus,
};

#[derive(
    Debug,
    Clone,
)]
pub struct Heartbeat {

    pub cycle: u64,

    pub active_tasks: usize,

    pub runtime_status: String,
}

pub fn runtime_heartbeat(
    tasks: &Vec<RuntimeTask>,
)
    -> Heartbeat
{

    Heartbeat {

        cycle: 1,

        active_tasks:
            tasks.len(),

        runtime_status:
            String::from(
                "operational"
            ),
    }
}

pub fn schedule_task(
    task: RuntimeTask,
)
{

    println!(
        "[SCHEDULER] queued task: {}",
        task.id
    );

    match task.status {

        TaskStatus::Pending => {

            println!(
                "[TASK STATUS] pending"
            );
        }

        TaskStatus::Running => {

            println!(
                "[TASK STATUS] running"
            );
        }

        TaskStatus::Completed => {

            println!(
                "[TASK STATUS] completed"
            );
        }

        TaskStatus::Failed => {

            println!(
                "[TASK STATUS] failed"
            );
        }
    }
}
