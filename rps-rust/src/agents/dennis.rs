use crate::agent;

pub struct Dennis {}

impl agent::Agent for Dennis {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Dennis",
            description: 
                "Dennis' professional agent",
            name: "Dennis Agent",
            enabled: true,
        }
    }

    fn play(&self, round: usize, _my: &str, opp: &str, _rnd: f64) -> String {
        // if (opp.chars().nth(0))
        let i : usize = round % 42;
        return "R".to_string();
        // return "RRRRPRRPRRRPRRRPRRRRPRRRRRPRRRRRRPRRRRRRRR"[i .. i+1].to_string();
        //return "RPS"[round % 3..round % 3 + 1].to_string();
    }
}
