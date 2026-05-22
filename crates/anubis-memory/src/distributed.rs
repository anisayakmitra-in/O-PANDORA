use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CognitionReplica {

    pub replica_id:
        String,

    pub node_id:
        String,

    pub last_synced_epoch:
        u64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct SynchronizationEvent {

    pub event_id:
        String,

    pub source_replica:
        String,

    pub target_replica:
        String,

    pub synchronized_at:
        u64,
}

pub struct SynchronizationEngine;

impl SynchronizationEngine {

    pub fn synchronize(

        source:
            &CognitionReplica,

        target:
            &CognitionReplica,

    ) -> SynchronizationEvent {

        SynchronizationEvent {

            event_id:
                format!(
                    "sync-{}-{}",
                    source.replica_id,
                    target.replica_id,
                ),

            source_replica:
                source
                    .replica_id
                    .clone(),

            target_replica:
                target
                    .replica_id
                    .clone(),

            synchronized_at:
                std::time
                    ::SystemTime
                    ::now()
                    .duration_since(
                        std::time
                            ::UNIX_EPOCH
                    )
                    .unwrap()
                    .as_secs(),
        }
    }
}

impl SynchronizationEngine {

    pub fn conflict_detected(

        source_epoch:
            u64,

        target_epoch:
            u64,

    ) -> bool {

        source_epoch
            !=
            target_epoch
    }
}
