use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RetrievalQuery {

    pub query_id:
        String,

    pub semantic_query:
        String,

    pub namespace:
        Option<String>,

    pub tags:
        Vec<String>,

    pub limit:
        usize,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RetrievalResult {

    pub memory_id:
        String,

    pub score:
        f32,

    pub matched_content:
        String,
}
