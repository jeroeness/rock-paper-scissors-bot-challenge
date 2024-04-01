use clap::Parser;

mod agent;
mod agents;

use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    agent: Option<String>,

    #[arg(short, long)]
    opponent: Option<String>,

    #[arg(short, long, default_value_t = 1000)]
    rounds: usize,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn get_agent(name: &str) -> Box<dyn agent::Agent> {
    match name {
        "cycler" => Box::new(agents::cycler::Cycler {}),
        "random" => Box::new(agents::random::Random {}),
        "rock" => Box::new(agents::rock::Rock {}),
        "phased_cycler" => Box::new(agents::phased_cycler::PhasedCycler {}),
        "reverse_cycler" => Box::new(agents::reverse_cycler::ReverseCycler {}),
        "learner" => Box::new(agents::learner::Learner {}),
        "mirror" => Box::new(agents::mirror::Mirror {}),
        "random_biased" => Box::new(agents::random_biased::RandomBiased {}),
        "random_counter" => Box::new(agents::random_counter::RandomCounter {}),
        "rock_scissors" => Box::new(agents::rock_scissors::RockScissors {}),
        "delayed_mirror" => Box::new(agents::delayed_mirror::DelayedMirror {}),

        _ => panic!("Unknown agent '{}'", name),
    }
}

fn get_all_agents() -> Vec<&'static str> {
    vec![
        "cycler",
        "random",
        "rock",
        "phased_cycler",
        "reverse_cycler",
        "learner",
        "mirror",
        "random_biased",
        "random_counter",
        "rock_scissors",
        "delayed_mirror",
    ]
}

fn battle_royale() {
    let mut total_score = 0;
    for agent1 in get_all_agents().iter() {
        for agent2 in get_all_agents().iter() {
            if agent1 == agent2 {
                continue;
            }

            let agent1 = get_agent(agent1);
            let agent2 = get_agent(agent2);
            let scores = match_agents(&agent1, &agent2, false, 1000);
            total_score += scores[0];
            println!(
                "{:>20} vs {:20} {:>4} : {:<4}",
                agent1.get_attributes().name,
                agent2.get_attributes().name,
                scores[0],
                scores[1]
            );
        }
    }
    println!(
        "Average score: {}",
        total_score as f64 / get_all_agents().len() as f64
    );
    // TODO keep scores of all agents
}

fn single_battle(agent1: &str, agent2: &str, verbose: bool, rounds: usize) -> [u64; 2] {
    let agent1 = get_agent(agent1);
    let agent2 = get_agent(agent2);
    match_agents(&agent1, &agent2, verbose, rounds)
}

fn one_to_all(agent: &str, verbose: bool, rounds: usize) {
    let mut total_score = 0;
    for opponent in get_all_agents()
        .iter()
        .filter(|agent| get_agent(agent).get_attributes().enabled)
        .collect::<Vec<_>>()
    {
        let scores = single_battle(&agent, opponent, verbose, rounds);
        total_score += scores[0];
        println!(
            "{:>20} vs {:20} {:>4} : {:<4}",
            agent, opponent, scores[0], scores[1]
        );
    }
    println!(
        "Average score: {}",
        total_score as f64 / get_all_agents().len() as f64
    );
}

fn main() {
    let args = Args::parse();

    match args.agent {
        Some(agent) => match args.opponent {
            Some(opponent) => {
                single_battle(&agent, &opponent, args.verbose, args.rounds);
            }
            None => {
                one_to_all(&agent, args.verbose, args.rounds);
            }
        },
        None => {
            battle_royale();
        }
    }
}

fn match_agents(
    agent1: &Box<dyn agent::Agent>,
    agent2: &Box<dyn agent::Agent>,
    verbose: bool,
    rounds: usize,
) -> [u64; 2] {
    let mut moves = ["".to_string(), "".to_string()];
    let mut scores: [u64; 2] = [0, 0];
    let mut draw_counter = 0;
    let mut rng = rand::thread_rng();

    for r in 1..=rounds {
        let m1 = agent1.play(r, &moves[0], &moves[1], rng.gen::<f64>());
        let m2 = agent2.play(r, &moves[1], &moves[0], rng.gen::<f64>());

        assert!(
            m1 == "R" || m1 == "P" || m1 == "S",
            "Invalid move '{}' in {}",
            m1,
            agent1.get_attributes().name
        );
        assert!(
            m2 == "R" || m2 == "P" || m2 == "S",
            "Invalid move '{}' in {}",
            m2,
            agent2.get_attributes().name
        );

        moves[0] += &m1;
        moves[1] += &m2;

        let mut verbose_score1 = " ";
        let mut verbose_score2 = " ";

        if m1 != m2 {
            draw_counter = 0;
        }

        if m1 == m2 {
            draw_counter += 1;
            if draw_counter > 50 {
                if verbose {
                    println!("Round {:4}: Draw limit reached", r);
                }
                break;
            }
        } else if m1 == "S" && m2 == "P" {
            scores[0] += 1;
            verbose_score1 = "*";
        } else if m1 == "P" && m2 == "R" {
            scores[0] += 1;
            verbose_score1 = "*";
        } else if m1 == "R" && m2 == "S" {
            scores[0] += 2;
            verbose_score1 = "*";
        } else if m2 == "R" {
            scores[1] += 2;
            verbose_score2 = "*";
        } else {
            scores[1] += 1;
            verbose_score2 = "*";
        }

        if verbose {
            println!(
                "Round {:4}: {}{}{}  vs {}{}{}",
                r, verbose_score1, m1, verbose_score1, verbose_score2, m2, verbose_score2
            );
        }
    }

    if verbose {
        println!("Scores: {} : {}", scores[0], scores[1]);
    }

    scores
}
