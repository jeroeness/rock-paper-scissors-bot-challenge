use crate::agent;

pub struct Random {}

impl agent::Agent for Random {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Random Agent",
            name: "Random",
            enabled: true,
        }
    }

    fn play(&self, _round: usize, _my: &str, _opp: &str, rnd: f64) -> String {
        let choices = ["R", "P", "S"];
        choices.get((rnd * 3.0) as usize).unwrap().to_string()
    }
}
