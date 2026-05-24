use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmAgent {
    pub agent_id: String,

    pub specialization: String,

    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmTask {
    pub task_id: String,

    pub objective: String,

    pub assigned_agent: Option<String>,
}

pub struct SwarmOrchestrator;

impl SwarmOrchestrator {
    pub fn coordinate(agents: &[SwarmAgent], tasks: &mut Vec<SwarmTask>) {
        for task in tasks.iter_mut() {
            if task.assigned_agent.is_some() {
                continue;
            }

            for agent in agents {
                if !agent.active {
                    continue;
                }

                task.assigned_agent = Some(agent.agent_id.clone());

                println!("[SWARM] {} -> {}", task.task_id, agent.agent_id);

                break;
            }
        }
    }
}
