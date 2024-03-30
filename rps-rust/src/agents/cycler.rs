use crate::agent;

pub struct Cycler {}

impl agent::Agent for Cycler {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Cycles through Rock, Paper, Scissors",
            name: "Cycler",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _: &str, _: &str, _: f64) -> String {
        let choices = ["R", "P", "S"];
        choices.get(round % 3).unwrap().to_string()
    }
}
