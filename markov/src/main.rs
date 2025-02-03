use fxhash::FxHashMap;
use rand::{rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead},
};
use tiktoken_rs::{o200k_base, CoreBPE};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkovModel {
    order: usize,
    transitions: FxHashMap<Vec<u32>, Vec<u32>>,
}

impl MarkovModel {
    pub fn new(order: usize) -> Self {
        if order == 0 {
            panic!("Order of Markov Model must be at least 1");
        }

        Self {
            order,
            transitions: FxHashMap::default(),
        }
    }

    pub fn train(&mut self, bpe: &CoreBPE, text: &str) {
        // let tokens: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

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

    pub fn generate(&self, bpe: &CoreBPE, start: String, length: usize) -> String {
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

    if args.len() < 3 {
        eprintln!("usage: {} train|complete <model>", &args[0]);
        return;
    }

    let action = &args[1];
    let path = &args[2];

    if action == "train" && args.len() < 4 {
        eprintln!("usage: {} train <model> <order>", &args[0]);
        return;
    }

    let mut model: MarkovModel = if fs::exists(path).is_ok_and(|v| v) {
        let file = File::open(path).unwrap();

        let buffered_reader = io::BufReader::new(file);
        let decoder = zstd::Decoder::new(buffered_reader).unwrap();

        let decoded = bincode::deserialize_from(decoder).unwrap();
        decoded
    } else {
        let order: usize = args[3].parse().unwrap();

        MarkovModel::new(order)
    };

    let bpe = o200k_base().unwrap();

    match action.as_str() {
        "train" => {
            // train
            let lock = io::stdin().lock();

            for line in lock.lines() {
                match line {
                    Ok(line) => model.train(&bpe, &line),
                    Err(_) => continue,
                }

                // eprintln!("[dbg] fed line '{}'", line);
            }

            let file = File::create(path).unwrap();
            let mut encoder = zstd::Encoder::new(&file, 22).unwrap();

            bincode::serialize_into(&mut encoder, &model).unwrap();

            encoder.finish().unwrap();

            eprintln!(
                "[dbg] model file written with {} transitions",
                model.transitions.len()
            );
        }
        "complete" => {
            let lock = io::stdin().lock();

            for line in lock.lines() {
                let line = line.unwrap();

                let completion = model.generate(&bpe, line, 8);

                println!("{}", completion);
            }
        }
        _ => {}
    }
}
