use std::time::Instant;

use crate::graph::{Graph, Edge};
use crate::parser::parse_graph;

use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

pub fn run_problems() {
    // Problem 1: Read precomputed CH graph and run CH queries
    run_problem_1();

    // Problem 2: Preprocessing germany graph (should take less than 5 minutes on 8-core)
    // and compare my CH performance (should take less than 10ms) with the provided one
    run_problem_2();
}

fn run_problem_1() -> Graph {
    println!("Running problem 1: Reading precomputed Stuttgart graph file and run CH");
    
    let start = Instant::now();

    let graph = read_precomputed_stuttgart_graph();

    let duration = start.elapsed();

    println!("Problem 1 done in {:?}", duration);

    return graph;
}

fn read_precomputed_stuttgart_graph() -> Graph {

    let path = "stgtregbz_ch.fmi";
    let graph = parse_graph(path);
    return graph;

}

fn run_problem_2() {
    println!("Running problem 2: CH Preprocessing");

    let start = Instant::now();

    // TODO

    let duration = start.elapsed();

    println!("Problem 2 time: {:?}", duration);
}
