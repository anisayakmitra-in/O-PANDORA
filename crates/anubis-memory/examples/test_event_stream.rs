use anubis_memory::category::{
    CognitionCategory,
};

use anubis_memory::graph::{
    MemoryGraph,
    MemoryNode,
    MemoryEdge,
    RelationshipType,
};

use anubis_memory::event::{
    CognitionEvent,
    CognitionEventStream,
};

use anubis_memory::temporal::{
    TemporalMetadata,
};

fn main() {

    let mut graph =
        MemoryGraph::default();

    CognitionEventStream
        ::emit(

            &mut graph,

            CognitionEvent
                ::NodeCreated(

                    MemoryNode {

                        node_id:
                            String::from(
                                "reasoning"
                            ),

                        namespace:
                            String::from(
                                "shadow"
                            ),

                        category:
                            CognitionCategory
                                ::Reasoning,

                        temporal:
                            TemporalMetadata {

                                timestamp:
                                    1000,

                                sequence:
                                    1,
                            },

                        label:
                            String::from(
                                "Reasoning"
                            ),

                        content:
                            String::from(
                                "Evaluate mutation"
                            ),
                    }
                )
        );

    CognitionEventStream
        ::emit(

            &mut graph,

            CognitionEvent
                ::NodeCreated(

                    MemoryNode {

                        node_id:
                            String::from(
                                "mutation"
                            ),

                        namespace:
                            String::from(
                                "gepa"
                            ),

                        category:
                            CognitionCategory
                                ::Mutation,

                        temporal:
                            TemporalMetadata {

                                timestamp:
                                    2000,

                                sequence:
                                    2,
                            },

                        label:
                            String::from(
                                "Mutation"
                            ),

                        content:
                            String::from(
                                "Prompt optimization"
                            ),
                    }
                )
        );

    CognitionEventStream
        ::emit(

            &mut graph,

            CognitionEvent
                ::EdgeCreated(

                    MemoryEdge {

                        edge_id:
                            String::from(
                                "edge-1"
                            ),

                        source:
                            String::from(
                                "reasoning"
                            ),

                        target:
                            String::from(
                                "mutation"
                            ),

                        relationship:
                            RelationshipType
                                ::Deliberation,

                        weight:
                            0.95,
                    }
                )
        );

    println!(
        "{:#?}",
        graph.neighbors(
            "reasoning"
        )
    );
}
