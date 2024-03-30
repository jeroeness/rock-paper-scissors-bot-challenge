use crate::agent;

pub struct RandomBiased {}

impl agent::Agent for RandomBiased {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Random Agent biased towards Rock",
            name: "Random Biased",
            enabled: true,
        }
    }

    fn play(&self, _round: usize, _my: &str, _opp: &str, rnd: f64) -> String {
        let choices = ["R", "P", "S", "R"];
        choices.get((rnd * 4.0) as usize).unwrap().to_string()
    }
}
