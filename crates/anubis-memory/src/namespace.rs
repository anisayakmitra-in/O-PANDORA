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
pub struct MemoryNamespace {

    pub namespace_id:
        String,

    pub owner:
        String,

    pub allowed_readers:
        Vec<String>,

    pub allowed_writers:
        Vec<String>,

    pub retention_policy:
        String,
}

pub struct NamespaceValidator;

impl NamespaceValidator {

    pub fn can_read(

        namespace:
            &MemoryNamespace,

        identity:
            &str,

    ) -> bool {

        namespace.owner == identity

        ||

        namespace
            .allowed_readers
            .contains(
                &identity.to_string()
            )
    }

    pub fn can_write(

        namespace:
            &MemoryNamespace,

        identity:
            &str,

    ) -> bool {

        namespace.owner == identity

        ||

        namespace
            .allowed_writers
            .contains(
                &identity.to_string()
            )
    }
}
