use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub enum SchedulerEvent {
    TaskQueued(Uuid),
    TaskStarted(Uuid),
    TaskCompleted(Uuid),
    TaskFailed(Uuid),
    TaskRetried(Uuid),
    TaskCancelled(Uuid),
    WatchdogTriggered(Uuid),
}
