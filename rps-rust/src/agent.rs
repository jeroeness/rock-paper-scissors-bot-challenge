pub struct AgentAttributes<'a> {
    pub author: &'a str,
    pub description: &'a str,
    pub name: &'a str,
    pub enabled: bool,
}

pub trait Agent {
    fn play(&self, round: usize, my: &str, opp: &str, rnd: f64) -> String;
    fn get_attributes(&self) -> AgentAttributes;
}

pub struct Battler<'a> {
    pub agent: &'a Box<dyn Agent>,
    pub scores: Vec<u64>,
    pub weighted_score: f64,
}
