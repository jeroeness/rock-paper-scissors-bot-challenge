use std::{collections::HashMap, sync::Mutex};

use itertools::Itertools;

use crate::agent::{self, Agent};

struct InternalState {
    possible_agents: Vec<Box<dyn Agent>>,
    previous_random: Option<f64>,
}

impl InternalState {
    fn new() -> Self {
        Self {
            possible_agents: get_initial_agents(),
            previous_random: None,
        }
    }

    fn _possible_agent_names(&self) -> Vec<&str> {
        self.possible_agents
            .iter()
            .map(|agent| agent.get_attributes().name)
            .collect()
    }
}

pub struct Ivo {
    internal_state: Mutex<InternalState>,
}

impl Ivo {
    pub fn new() -> Self {
        Self {
            internal_state: Mutex::new(InternalState::new()),
        }
    }
}

impl agent::Agent for Ivo {
    fn get_attributes(&self) -> agent::AgentAttributes {
        agent::AgentAttributes {
            author: "Ivo",
            description: "Ivo's amature agent",
            name: "Ivo agent",
            enabled: true,
        }
    }

    fn play(&self, round: usize, me: &str, opponent: &str, random: f64) -> String {
        let mut internal_state = self.internal_state.try_lock().expect(
            format!(
                "Expected that {} is the only one to use this mutex",
                self.get_attributes().name
            )
            .as_str(),
        );

        // dbg!(round);

        if round == 1 {
            *internal_state = InternalState::new()
        } else {
            let to_be_removed: Vec<usize> =
                internal_state
                    .possible_agents
                    .iter()
                    .enumerate()
                    .filter_map(|(index, agent)| {
                        let expected_move = agent.play(
                        round - 1,
                        &opponent[..opponent.len() - 1],
                        &me[..me.len() - 1],
                        internal_state.previous_random.expect(
                            "Expected to have a previous random value in rounds greater than 1",
                        ),
                    ).chars().last().expect("Expected play to have returned a value");

                        let actual_move = opponent
                            .chars()
                            .last()
                            .expect("Expected opponent to have played a move");
                        if expected_move != actual_move {
                            // println!("Mismatch: opponent {:20} played {actual_move:#?} but was expecting {expected_move:#?}", agent.get_attributes().name);
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .collect();
            if !to_be_removed.is_empty() {
                for index in to_be_removed.iter().sorted().rev() {
                    internal_state.possible_agents.swap_remove(*index);
                }
                // println!(
                //     "Removed agents. Current agents: {:?}",
                //     internal_state.possible_agent_names(),
                // );
            }
        }
        internal_state.previous_random = Some(random);

        let mut moves = HashMap::from([("R", 0), ("P", 0), ("S", 0)]);

        for agent in internal_state.possible_agents.iter() {
            *moves
                .get_mut(agent.play(round, opponent, me, random).as_str())
                .expect("Expected agent to play a valid move") += 1;
        }

        let expected_move = moves
            .into_iter()
            .max_by_key(|(_move, count)| *count)
            .expect("Expected at least one value in the hashmap")
            .0
            .to_string();

        let counter = match expected_move.as_str() {
            "R" => "P",
            "P" => "S",
            "S" => "R",
            other => panic!("Unexpected expected_move: {other}"),
        };

        // println!("Expecting move {expected_move}, countering with {counter}");
        counter.to_string()
    }
}

fn get_initial_agents() -> Vec<Box<dyn Agent>> {
    let agents: Vec<Box<dyn Agent>> = vec![
        Box::new(crate::agents::cycler::Cycler {}),
        Box::new(crate::agents::random::Random {}),
        Box::new(crate::agents::rock::Rock {}),
        Box::new(crate::agents::phased_cycler::PhasedCycler {}),
        Box::new(crate::agents::reverse_cycler::ReverseCycler {}),
        Box::new(crate::agents::learner::Learner {}),
        Box::new(crate::agents::mirror::Mirror {}),
        Box::new(crate::agents::random_biased::RandomBiased {}),
        Box::new(crate::agents::random_counter::RandomCounter {}),
        Box::new(crate::agents::rock_scissors::RockScissors {}),
        Box::new(crate::agents::delayed_mirror::DelayedMirror {}),
        Box::new(crate::agents::dennis::Dennis {}),
    ];
    agents
        .into_iter()
        .filter(|agent| agent.get_attributes().enabled)
        .collect()
}
