use std::time::Instant;

use crate::graph::Graph;
use crate::parser::{parse_graph, parse_preprocessed_graph};
use crate::working_graph::{CHBuilder, WorkingGraph};

use rand::Rng;
use rand::thread_rng;

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
    let graph = parse_preprocessed_graph(path);

    let duration = start.elapsed();

    println!("Reading precomputed Stuttgart graph took {:?}", duration);

    return graph;
}

fn run_ch_query(graph: &Graph) {
    println!("Running CH query on stuttgart graph");

    // Generate random source and target nodes
    let mut rng = thread_rng();
    let n = graph.num_nodes;
    let source = rng.gen_range(0..n);
    let target = rng.gen_range(0..n);

    // Execute CH and print runtime
    let start_ch = Instant::now();
    let distance_ch = graph.ch_query(source, target);
    let duration_ch = start_ch.elapsed();
    println!("CH runtime: {:?}", duration_ch);

    // Execute CH with stall-on-demand and print runtime
    let start_ch_sod = Instant::now();
    let distance_ch_sod = graph.ch_query_with_sod(source, target);
    let duration_ch_sod = start_ch_sod.elapsed();
    println!("CH with stall-on-demand runtime: {:?}", duration_ch_sod);

    // Execute dijkstra and print runtime
    let start_dijkstra = Instant::now();
    let distance_dijsktra = graph.dijkstra_distance(source, target);
    let duration_dijkstra = start_dijkstra.elapsed();
    println!("Dijkstra runtime: {:?}", duration_dijkstra);

    // Ensure the computed distances are equal
    assert_eq!(distance_ch.unwrap(), distance_ch_sod.unwrap());
    assert_eq!(distance_ch.unwrap(), distance_dijsktra);
}

fn run_problem_2() {
    println!("Running problem 2: CH Preprocessing");
    let graph = read_germany_graph();
    let working_graph = preprocess_graph(&graph);
    run_ch_query(&graph);
}

fn read_germany_graph() -> Graph {
    let start = Instant::now();
    let path = "core/data/graphs/germany.fmi";
    let graph = parse_graph(path);
    let duration = start.elapsed();

    println!("Reading germany graph took {:?}", duration);

    return graph;
}

fn preprocess_graph(graph: &Graph) -> WorkingGraph {
    let start = Instant::now();
    let mut ch_builder = CHBuilder::from_graph(graph);
    ch_builder.run();
    let duration = start.elapsed();

    println!("Preprocessing germany graph took {:?}", duration);

    return ch_builder.g;
}
