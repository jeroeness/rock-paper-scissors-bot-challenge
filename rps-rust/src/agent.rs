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
    pub scores: Vec<f64>,
    pub weighted_score: f64,
}

#[derive(Debug)] #[derive(PartialEq)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

fn handScore(my: Hand, op: Hand) -> i32 {
    return match (my as i32 - op as i32) % 3 {
        0 => 0,
        1 => 1,
        _ => -1,
    };
}

fn counter(hand: Hand) -> Hand {
    return match (hand) {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    }
}

// The function returns 0 when both hands are the same.
#[test]
fn test_hand_scoring() {
    assert_eq!(handScore(Hand::Rock, Hand::Rock), 0);
    assert_eq!(handScore(Hand::Rock, Hand::Paper), -1);
    assert_eq!(counter(Hand::Rock), Hand::Paper);
    assert_eq!(parseHand("R".to_string()), Hand::Rock);
}

fn parseHand(str: String) -> Hand {
    return match (str.as_str()) {
        "R" => Hand::Rock,
        "S" => Hand::Scissors,
        "P" => Hand::Paper,
        _ => panic!("unknown hand"),
    }
}
