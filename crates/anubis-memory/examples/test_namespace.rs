use anubis_memory::namespace::{
    MemoryNamespace,
    NamespaceValidator,
};

fn main() {

    let namespace =
        MemoryNamespace {

            namespace_id:
                String::from(
                    "security.audit"
                ),

            owner:
                String::from(
                    "god-eye"
                ),

            allowed_readers:
                vec![

                    String::from(
                        "shadow-council"
                    ),
                ],

            allowed_writers:
                vec![

                    String::from(
                        "god-eye"
                    ),
                ],

            retention_policy:
                String::from(
                    "persistent"
                ),
        };

    println!(
        "shadow-council can read: {}",
        NamespaceValidator
            ::can_read(
                &namespace,
                "shadow-council"
            )
    );

    println!(
        "trading-gene can write: {}",
        NamespaceValidator
            ::can_write(
                &namespace,
                "trading-gene"
            )
    );
}
