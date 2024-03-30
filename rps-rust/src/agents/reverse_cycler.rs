use crate::agent;

pub struct ReverseCycler {}

impl agent::Agent for ReverseCycler {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Cycles through Scissors, Rock, Paper",
            name: "Cycler",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _: &str, _: &str, _: f64) -> String {
        let choices = ["S", "R", "P"];
        choices.get(round % 3).unwrap().to_string()
    }
}
