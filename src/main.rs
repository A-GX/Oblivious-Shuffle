#![allow(unused)]
pub mod shuffling_tests;
pub mod shuffling_utils;
pub mod debug;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = std::env::args().collect::<Vec<String>>();
    let party = Some(args[1].parse::<usize>().unwrap());
    shuffling_tests::run_test(party);
}