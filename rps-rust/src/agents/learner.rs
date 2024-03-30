use crate::agent;

pub struct Learner {}

impl agent::Agent for Learner {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Mirrors the opponent's second previous move",
            name: "Delayed Mirror",
            enabled: true,
        }
    }

    fn play(&self, round: usize, my: &str, opp: &str, _rnd: f64) -> String {
        if opp.len() < 3 {
            return "R".to_string();
        }
        for i in (1..=(std::cmp::min(50, opp.len() / 2))).rev() {
            let pattern = &opp[opp.len() - i..];
            let search = &opp[..opp.len() - 1];
            if let Some(index) = search.rfind(pattern) {
                let predict = &opp.chars().nth(index + pattern.len()).unwrap();
                return match predict {
                    'R' => "P".to_string(),
                    'P' => "S".to_string(),
                    _ => "R".to_string(),
                };
            }
        }
        return "RPS"[round % 3..round % 3 + 1].to_string();
    }
}
