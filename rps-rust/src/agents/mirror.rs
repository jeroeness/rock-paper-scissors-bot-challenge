use crate::agent;

pub struct Mirror {}

impl agent::Agent for Mirror {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Mirrors the opponent's previous move",
            name: "Mirror",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _my: &str, opp: &str, _rnd: f64) -> String {
        if round < 2 {
            return String::from("R");
        } else {
            return String::from(&opp[round - 2..round - 1]);
        }
    }
}
