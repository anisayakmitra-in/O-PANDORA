use anubis_memory::distributed::{
    CognitionReplica,
    SynchronizationEngine,
};

fn main() {

    let replica_a =
        CognitionReplica {

            replica_id:
                String::from(
                    "replica-a"
                ),

            node_id:
                String::from(
                    "node-1"
                ),

            last_synced_epoch:
                10,
        };

    let replica_b =
        CognitionReplica {

            replica_id:
                String::from(
                    "replica-b"
                ),

            node_id:
                String::from(
                    "node-2"
                ),

            last_synced_epoch:
                12,
        };

    let event =

        SynchronizationEngine
            ::synchronize(
                &replica_a,
                &replica_b,
            );

    println!(
        "{:#?}",
        event
    );

    let conflict =

        SynchronizationEngine
            ::conflict_detected(
                replica_a
                    .last_synced_epoch,

                replica_b
                    .last_synced_epoch,
            );

    println!(
        "Conflict detected: {}",
        conflict
    );
}
