use crate::agent;

pub struct Rock {}

impl agent::Agent for Rock {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Always Rocks",
            name: "Rock",
            enabled: true,
        }
    }

    fn play(&self, _round: usize, _: &str, _: &str, _rnd: f64) -> String {
        "R".to_string()
    }
}
