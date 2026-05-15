use serde::{
    Serialize,
    de::DeserializeOwned,
};

pub trait Signature {

    type Input:
        Serialize
        + Send
        + Sync;

    type Output:
        DeserializeOwned
        + Send
        + Sync;

    fn instruction()
        -> &'static str;
}

pub mod examples;
