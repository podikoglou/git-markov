use rand::{rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead},
};
use tiktoken_rs::o200k_base;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkovModel {
    order: usize,
    transitions: HashMap<Vec<u32>, Vec<u32>>,
}

impl MarkovModel {
    pub fn new(order: usize) -> Self {
        if order == 0 {
            panic!("Order of Markov Model must be at least 1");
        }
        Self {
            order,
            transitions: HashMap::new(),
        }
    }

    pub fn train(&mut self, text: &str) {
        // let tokens: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

        let bpe = o200k_base().unwrap();
        let tokens = bpe.encode_with_special_tokens(text);

        if tokens.len() <= self.order {
            return;
        }

        for window in tokens.windows(self.order + 1) {
            let state = window[..self.order].to_vec();
            let next_token = window[self.order];

            self.transitions
                .entry(state)
                .or_insert_with(Vec::new)
                .push(next_token);
        }
    }

    pub fn generate(&self, start: String, length: usize) -> String {
        let bpe = o200k_base().unwrap();
        let start = bpe.encode_with_special_tokens(&start);

        if start.len() != self.order {
            panic!("Start state must have exactly {} tokens", self.order);
        }

        let mut rng = rng();

        let mut current_state = start.to_vec();
        let mut output = current_state.clone();

        for _ in 0..length {
            if let Some(possible_next) = self.transitions.get(&current_state) {
                if let Some(next) = possible_next.choose(&mut rng) {
                    output.push(*next);
                    current_state.remove(0);
                    current_state.push(*next);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        bpe.decode(output).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let action = &args[1];
    let path = &args[2];

    let mut model: MarkovModel = if fs::exists(path).is_ok_and(|v| v) {
        let contents = fs::read(path).unwrap();
        bincode::deserialize(&contents).unwrap()
    } else {
        MarkovModel::new(2)
    };

    match action.as_str() {
        "train" => {
            // train
            let lock = io::stdin().lock();

            for line in lock.lines() {
                let line = line.unwrap();

                model.train(&line);

                eprintln!("[dbg] fed line '{}'", line);
            }

            fs::write(path, bincode::serialize(&model).unwrap()).unwrap();
            eprintln!(
                "[dbg] model file written with {} transitions",
                model.transitions.len()
            );
        }
        "complete" => {
            let lock = io::stdin().lock();

            for line in lock.lines() {
                let line = line.unwrap();

                let completion = model.generate(line, 8);

                println!("{}", completion);
            }
        }
        _ => {}
    }
}
