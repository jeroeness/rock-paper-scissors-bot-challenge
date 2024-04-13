use crate::agent;

pub struct Ivo {}

impl agent::Agent for Ivo {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Ivo",
            description: "Ivo's amature agent",
            name: "Ivo agent",
            enabled: false,
        }
    }

    fn play(&self, round: usize, _: &str, _: &str, _: f64) -> String {
        let choices = ["R", "P", "S"];
        choices.get(round % 3).unwrap().to_string()
    }
}
