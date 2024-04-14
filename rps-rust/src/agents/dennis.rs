use crate::agent;

pub struct Dennis {}

impl agent::Agent for Dennis {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Dennis",
            description: 
                "Dennis' professional agent",
            name: "Dennis Agent",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _my: &str, opp: &str, _rnd: f64) -> String {
        return "PS"[round % 2..round % 2 + 1].to_string();
    }
}
