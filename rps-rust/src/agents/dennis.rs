use crate::agent;

pub struct Dennis {}

impl agent::Agent for Dennis {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Dennis",
            description: "Dennis' amature agent",
            name: "Dennis Agent",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _: &str, _: &str, _: f64) -> String {
        let choices = ["R", "P", "S"];
        choices.get(round % 3).unwrap().to_string()
    }
}
