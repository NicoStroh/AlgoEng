use std::time::Instant;

mod parser;
mod graph;

use parser::parse_graph;

fn main() {
    // Problem 1: Read germany graph file
    run_task_1();

    // Problem 2: Determine weakly connected components
    run_task_2();

    // Problem 3: Permuting nodes randomly
    run_task_3();
}

fn run_task_1() {
    println!("Running Task 1: Reading germany graph file");
    
    let start = Instant::now();

    read_germany_graph();

    let duration = start.elapsed();

    println!("Task 1 done in {:?}", duration);
}

fn read_germany_graph() {

    let path = "/Users/nicostrohbach/AlgoEng/graphs/germany.fmi";
    parse_graph(path);

}

fn run_task_2() {
    println!("Running Task 2: Weakly Connected Components");

    let start = Instant::now();

    compute_weakly_connected_components();

    let duration = start.elapsed();

    println!("Task 2 time: {:?}", duration);
}

fn compute_weakly_connected_components() {

}

fn run_task_3() {
    println!("Running Task 3: Permuting nodes randomly");

    let start = Instant::now();

    permuting_nodes_randomly();

    let duration = start.elapsed();

    println!("Task 3 time: {:?}", duration);
}

fn permuting_nodes_randomly() {

}