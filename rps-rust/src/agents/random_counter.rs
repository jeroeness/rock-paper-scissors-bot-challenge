use crate::agent;

pub struct RandomCounter {}

impl agent::Agent for RandomCounter {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Jeroen",
            description: "Randomly plays Rock, Paper, Scissors, but tries to counter the opponent",
            name: "Random Counter",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _my: &str, opp: &str, rnd: f64) -> String {
        if round < 3 {
            return "RPS".chars().nth((rnd * 3.0) as usize).unwrap().to_string();
        }
        let opp = &opp[opp.len() - std::cmp::min(30, opp.len())..];
        let pred = opp.chars().nth((rnd * opp.len() as f64) as usize).unwrap();
        return match pred {
            'R' => "P".to_string(),
            'P' => "S".to_string(),
            _ => "R".to_string(),
        };
    }
}
