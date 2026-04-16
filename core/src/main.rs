use std::time::Instant;

mod parser;
mod graph;

use crate::graph::{Graph, Edge};
use parser::parse_graph;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    // Problem 1: Read germany graph file
    let graph = run_problem_1();

    // Problem 2: Determine weakly connected components
    run_problem_2(&graph);

    // Problem 3: Permuting nodes randomly
    run_problem_3(&graph);
}

fn run_problem_1() -> Graph {
    println!("Running problem 1: Reading germany graph file");
    
    let start = Instant::now();

    let graph = read_germany_graph();

    let duration = start.elapsed();

    println!("Problem 1 done in {:?}", duration);

    return graph;
}

fn read_germany_graph() -> Graph {

    let path = "/Users/nicostrohbach/AlgoEng/graphs/germany.fmi";
    let graph = parse_graph(path);
    return graph;

}

fn run_problem_2(graph: &Graph) {
    println!("Running problem 2: Weakly Connected Components");

    let start = Instant::now();

    compute_weakly_connected_components(&graph);

    let duration = start.elapsed();

    println!("Problem 2 time: {:?}", duration);
}

fn compute_weakly_connected_components(graph: &Graph) -> usize {
    let n = graph.num_nodes;

    let mut visited = vec![false; n];
    let mut count = 0;

    for start in 0..n {
        if !visited[start] {
            count += 1;
            dfs(graph, start, &mut visited);
        }
    }

    println!("Weakly connected components: {}", count);
    return count;
}

fn dfs(graph: &Graph, start: usize, visited: &mut [bool]) {
    let mut stack = vec![start];
    visited[start] = true;

    while let Some(node) = stack.pop() {
        // outgoing edges
        for edge in graph.outgoing(node) {
            let v = edge.target as usize;
            if !visited[v] {
                visited[v] = true;
                stack.push(v);
            }
        }

        // incoming edges (macht es "weakly")
        for edge in graph.incoming(node) {
            let v = edge.target as usize;
            if !visited[v] {
                visited[v] = true;
                stack.push(v);
            }
        }
    }
}

// Runtime with normal permutation: 4.6s
// Runtime with random permutation: 13.9s
// Conclusion: It is much slower with random permutation.
fn run_problem_3(graph: &Graph) {
    println!("Running problem 3: Permuting nodes randomly");

    let new_graph = permuting_nodes_randomly(&graph);

    let start = Instant::now();

    compute_weakly_connected_components(&new_graph);

    let duration = start.elapsed();

    println!("Problem 3 time: {:?}", duration);
}

fn permuting_nodes_randomly(graph: &Graph) -> Graph {
        let n = graph.num_nodes;

    // Crete random permutation
    let mut perm: Vec<usize> = (0..n).collect();
    let mut rng = thread_rng();
    perm.shuffle(&mut rng);

    // inverse Permutation: old -> new
    let mut new_index = vec![0usize; n];
    for (new_i, &old_i) in perm.iter().enumerate() {
        new_index[old_i] = new_i;
    }

    // collect new edge lists
    let mut new_out_edges: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut new_in_edges: Vec<Vec<Edge>> = vec![Vec::new(); n];

    for u in 0..n {
        let new_u = new_index[u];

        // outgoing
        for e in graph.outgoing(u) {
            let v = e.target;
            let new_v = new_index[v];

            new_out_edges[new_u].push(Edge {
                target: new_v,
                weight: e.weight,
            });
        }

        // incoming
        for e in graph.incoming(u) {
            let v = e.target;
            let new_v = new_index[v];

            new_in_edges[new_u].push(Edge {
                target: new_v,
                weight: e.weight,
            });
        }
    }

    // CSR flatten
    fn build_csr(adj: Vec<Vec<Edge>>) -> (Vec<Edge>, Vec<usize>) {
        let mut offsets = vec![0usize];
        let mut edges = Vec::new();

        for bucket in adj {
            edges.extend(bucket);
            offsets.push(edges.len());
        }

        (edges, offsets)
    }

    let outgoing = build_csr(new_out_edges);
    let incoming = build_csr(new_in_edges);

    return Graph::new(n, outgoing, incoming);
}