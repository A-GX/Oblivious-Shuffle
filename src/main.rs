#![allow(unused)]
mod shuffling_tests;
use shuffling_tests::{W_1, W_2, S_1, S_2};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let party = Some(args[1].parse::<usize>().unwrap());
    shuffling_tests::run_test(party);
}