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
pub struct NervousSignal {

    pub origin:
        String,

    pub signal:
        String,

    pub urgency:
        f32,
}

pub struct SwarmNervousSystem;

impl SwarmNervousSystem {

    pub fn propagate(

        signals:
            &[NervousSignal],
    )
    {

        for signal
            in signals
        {

            println!(
                "[NERVOUS] {} -> {} urgency={}",
                signal.origin,
                signal.signal,
                signal.urgency
            );

            if signal.urgency
                > 0.90
            {

                println!(
                    "[NERVOUS] global reflex triggered"
                );
            }
        }
    }
}
