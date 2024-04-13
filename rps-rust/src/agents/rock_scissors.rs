use crate::agent;

pub struct RockScissors {}

impl agent::Agent for RockScissors {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Cycles through Rock, Scissors",
            name: "Rock Scissors",
            enabled: true,
        }
    }

    fn play(&self, _round: usize, _: &str, _: &str, rnd: f64) -> String {
        return "RS".chars().nth((rnd * 2.0) as usize).unwrap().to_string();
    }
}
