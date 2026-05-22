use crate::namespace_registry::NamespaceRecord;

pub fn validate_namespace(

    namespace:
        &NamespaceRecord,
)
    -> bool
{

    namespace.isolated
}
