use crate::agent;

pub struct DelayedMirror {}

impl agent::Agent for DelayedMirror {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Mirrors the opponent's second previous move",
            name: "Delayed Mirror",
            enabled: true,
        }
    }

    fn play(&self, round: usize, my: &str, opp: &str, rnd: f64) -> String {
        if round == 1 {
            return if rnd < 0.5 {
                "P".to_string()
            } else {
                "R".to_string()
            };
        }
        if round == 2 {
            return "P".to_string();
        }
        if round <= 10 && opp.chars().nth(round - 2) == my.chars().nth(round - 2) {
            return if rnd < 0.5 {
                "P".to_string()
            } else {
                "R".to_string()
            };
        }
        if round <= 10
            && opp.chars().nth(round - 2) != my.chars().nth(round - 2)
            && opp.chars().nth(round - 2) != Some('S')
        {
            return if opp.chars().nth(round - 2) == Some('R') {
                "R".to_string()
            } else {
                "P".to_string()
            };
        }
        return opp.chars().nth(round - 3).unwrap().to_string();
    }
}
