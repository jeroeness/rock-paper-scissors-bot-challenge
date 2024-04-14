use clap::Parser;

mod agent;
mod agents;

use rand::Rng;
use std::time::Instant;

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
        "scissors" => Box::new(agents::scissors::Scissors {}),
        "delayed_mirror" => Box::new(agents::delayed_mirror::DelayedMirror {}),
        "dennis" => Box::new(agents::dennis::Dennis {}),
        "ivo" => Box::new(agents::ivo::Ivo::new()),

        _ => panic!("Unknown agent '{}'", name),
    }
}

fn get_all_agents() -> Vec<Box<dyn agent::Agent>> {
    vec![
        "rock",
        "cycler",
        "random",
        "phased_cycler",
        "reverse_cycler",
        "learner",
        "mirror",
        "random_biased",
        "random_counter",
        "rock_scissors",
        "scissors",
        "delayed_mirror",
        "ivo",
        "dennis",
        ]
    .into_iter()
    .map(|s| get_agent(s))
    .filter(|a| a.get_attributes().enabled)
    .collect()
}

fn battle_royale() {
    let binding = get_all_agents();
    let rounds = 1000;
    let mut battlers = binding
        .iter()
        .map(|a| agent::Battler {
            agent: a,
            scores: vec![],
            weighted_score: 0.0,
        })
        .collect::<Vec<_>>();

    for agent1 in 0..battlers.len() {
        for agent2 in agent1..battlers.len() {
            let mut avg_score = [0.0, 0.0];
            for trial in 0..9 {
                println!(
                    "Starting trial {trial} between {} and {}",
                    battlers[agent1].agent.get_attributes().name,
                    battlers[agent2].agent.get_attributes().name
                );
                let scores = match_agents(
                    vec![battlers[agent1].agent, battlers[agent2].agent],
                    false,
                    rounds,
                );
                let scores2 = match_agents(
                    vec![battlers[agent2].agent, battlers[agent1].agent],
                    false,
                    rounds,
                );
                avg_score[0] += (scores[0] + scores2[1]) as f64 / 2.0;
                avg_score[1] += (scores[1] + scores2[0]) as f64 / 2.0;
            }
            battlers[agent1].scores.push(avg_score[0] / 10.0);
            battlers[agent2].scores.push(avg_score[1] / 10.0);
        }
    }
    for i in 0..battlers.len() {
        battlers[i].weighted_score = battlers[i].scores.iter().sum::<f64>() / battlers.len() as f64;
    }
    // for b in battlers.iter_mut() {
    //     b.weighted_score = 400.0;
    //     println!("{:<20} {:?}", b.agent.get_attributes().name, b.scores)
    // }
    // for _ in 0..10 {
    //     for i in 0..battlers.len() {
    //         let sum_of_weights: f64 = battlers.iter().map(|b| b.weighted_score).sum();
    //         // let total: u64 = battlers[i].scores.iter().map(|s| s).sum();
    //         let mut total = 0.0;
    //         for j in 0..battlers.len() {
    //             total +=
    //                 battlers[i].scores[j] as f64 * (battlers[j].weighted_score / rounds as f64);
    //         }
    //         battlers[i].weighted_score = total as f64 / sum_of_weights * rounds as f64;
    //     }
    // }
    battlers.sort_by(|a, b| b.weighted_score.partial_cmp(&a.weighted_score).unwrap());
    println!("Rank  Agent                Author     Score");
    println!("====  ==================   =========  =====");
    for i in 0..battlers.len() {
        println!(
            " #{:<3} {:<20} {:<10} {:>4.1}",
            i + 1,
            battlers[i].agent.get_attributes().name,
            battlers[i].agent.get_attributes().author,
            battlers[i].weighted_score
        );
    }
}

fn one_to_all(agent: &Box<dyn agent::Agent>, verbose: bool, rounds: usize) {
    let mut total_score = 0;
    for opponent in get_all_agents()
        .iter()
        .filter(|agent| agent.get_attributes().enabled)
        .collect::<Vec<_>>()
    {
        let scores = match_agents(vec![&agent, &opponent], verbose, rounds);
        total_score += scores[0];
        println!(
            "{:>20} vs {:20} {:>4} : {:<4}",
            agent.get_attributes().name,
            opponent.get_attributes().name,
            scores[0],
            scores[1]
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
                let scores = match_agents(
                    vec![&get_agent(&agent), &get_agent(&opponent)],
                    args.verbose,
                    args.rounds,
                );
                println!(
                    "{:>20} vs {:20} {:>4} : {:<4}",
                    agent, opponent, scores[0], scores[1]
                );
            }
            None => {
                one_to_all(&get_agent(&agent), args.verbose, args.rounds);
            }
        },
        None => {
            battle_royale();
        }
    }
}

fn match_agents(agents: Vec<&Box<dyn agent::Agent>>, verbose: bool, rounds: usize) -> [u64; 2] {
    assert!(agents.len() == 2);

    let mut moves = ["".to_string(), "".to_string()];
    let mut scores: [u64; 2] = [0, 0];
    let mut draw_counter = 0;
    let mut rng = rand::thread_rng();

    for r in 1..=rounds {
        let mut mvs = vec!["".to_string(); 2];
        for i in 0..2 {
            let agent = &agents[i];
            let start_time = Instant::now();
            let mv = agent.play(r, &moves[0], &moves[1], rng.gen::<f64>());
            let elapsed_time = start_time.elapsed();
            assert!(
                elapsed_time < std::time::Duration::from_millis(200),
                "Agent '{}' took too much thinking time: {:?}",
                agent.get_attributes().name,
                elapsed_time
            );
            assert!(
                mv == "R" || mv == "P" || mv == "S",
                "Invalid move '{}' in {}",
                mv,
                agent.get_attributes().name
            );
            moves[i] += &mv;
            mvs[i] = mv;
        }

        let mut verbose_score1 = " ";
        let mut verbose_score2 = " ";

        if mvs[0] != mvs[1] {
            draw_counter = 0;
        }

        if mvs[0] == mvs[1] {
            draw_counter += 1;
            if draw_counter > 50 {
                if verbose {
                    println!("Round {:4}: Draw limit reached", r);
                }
                break;
            }
        } else if mvs[0] == "S" && mvs[1] == "P" {
            scores[0] += 1;
            verbose_score1 = "*";
        } else if mvs[0] == "P" && mvs[1] == "R" {
            scores[0] += 1;
            verbose_score1 = "*";
        } else if mvs[0] == "R" && mvs[1] == "S" {
            scores[0] += 2;
            verbose_score1 = "*";
        } else if mvs[1] == "R" {
            scores[1] += 2;
            verbose_score2 = "*";
        } else {
            scores[1] += 1;
            verbose_score2 = "*";
        }

        if verbose {
            println!(
                "Round {:4}: {}{}{}  vs {}{}{}",
                r, verbose_score1, mvs[0], verbose_score1, verbose_score2, mvs[1], verbose_score2
            );
        }
    }

    if verbose {
        println!("Scores: {} : {}", scores[0], scores[1]);
    }

    scores
}
