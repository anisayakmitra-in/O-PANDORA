use pandora_scheduler::task::{
    ExecutionTier,
    Task,
    TaskPayload,
};

#[tokio::main]
async fn main() {

    let task =
        Task::new(
            ExecutionTier::Tier1Isolated,

            TaskPayload::ExecuteCommand {

                command: vec![
                    "echo".to_string(),
                    "hello-pandora".to_string(),
                ],
            },
        );

    println!(
        "TASK CREATED: {:?}",
        task.id
    );
}

