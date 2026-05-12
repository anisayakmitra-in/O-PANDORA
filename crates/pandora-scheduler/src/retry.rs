use crate::task::{Recurrence, Task, TaskStatus};
use chrono::Utc;

pub fn apply_retry_policy(task: &mut Task) {
    if task.attempts >= task.budget.max_retries || task.attempts >= task.retry_policy.max_attempts {
        task.status = TaskStatus::Failed;
        return;
    }

    let backoff = if task.retry_policy.exponential_backoff {
        task.retry_policy.backoff_seconds * 2_u64.pow(task.attempts - 1)
    } else {
        task.retry_policy.backoff_seconds
    };

    task.next_run = Utc::now() + chrono::Duration::seconds(backoff as i64);
    task.status = TaskStatus::Pending;
}

pub fn calculate_next_recurrence(task: &mut Task) {
    if task.invocations >= task.budget.max_invocations {
        task.status = TaskStatus::Completed;
        return;
    }

    match &task.recurrence {
        Recurrence::OneShot => {
            task.status = TaskStatus::Completed;
        }
        Recurrence::IntervalSeconds(secs) => {
            task.next_run = Utc::now() + chrono::Duration::seconds(*secs as i64);
            task.status = TaskStatus::Pending;
        }
        Recurrence::Cron(_) => {
            // Stub: Integrate `cron` crate logic here later
            task.status = TaskStatus::Completed; 
        }
    }
}
