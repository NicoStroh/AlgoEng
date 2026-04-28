use std::time::Instant;

use crate::graph::{Graph};
use crate::parser::parse_graph;

pub fn run_problems() {
    // Problem 1: Read precomputed CH graph and run CH queries
    run_problem_1();

    // Problem 2: Preprocessing germany graph (should take less than 5 minutes on 8-core)
    // and compare my CH performance (should take less than 10ms) with the provided one
    run_problem_2();
}

fn run_problem_1() -> Graph {
    println!("Running problem 1: Reading precomputed Stuttgart graph file and run CH");
    

    let graph = read_precomputed_stuttgart_graph();
    run_ch_query(&graph);

    return graph;
}

fn read_precomputed_stuttgart_graph() -> Graph {

    let start = Instant::now();

    let path = "core/data/graphs/stgtregbz_ch.fmi";
    let graph = parse_graph(path);

    let duration = start.elapsed();

    println!("Reading precomputed Stuttgart graph took {:?} seconds", duration);

    return graph;

}

fn run_ch_query(graph: &Graph) {
    println!("Running CH query on stuttgart graph");

    let source = 10usize;
    let target = 1000usize;

    let start = Instant::now();

    graph.ch_query(source, target);

    let duration = start.elapsed();

    println!("CH runtime: {:?}", duration);


    let start = Instant::now();

    graph.dijkstra_distance(source, target);

    let duration = start.elapsed();

    println!("Dijkstra runtime: {:?}", duration);
}

fn run_problem_2() {
    println!("Running problem 2: CH Preprocessing");

    let start = Instant::now();

    // TODO

    let duration = start.elapsed();

    println!("Problem 2 time: {:?}", duration);
}
