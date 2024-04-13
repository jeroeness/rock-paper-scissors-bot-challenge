use crate::agent;

pub struct Scissors {}

impl agent::Agent for Scissors {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Dennis",
            description: "Always cuts",
            name: "Scissors",
            enabled: true,
        }
    }

    fn play(&self, _round: usize, _: &str, _: &str, _rnd: f64) -> String {
        "S".to_string()
    }
}
