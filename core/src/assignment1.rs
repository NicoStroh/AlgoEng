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
    // Problem 1: Read germany graph file
    let graph = run_problem_1();

    // Problem 2: Determine weakly connected components
    run_problem_2(&graph);

    // Problem 3: Permuting nodes randomly
    //run_problem_3(&graph);

    // Problem 4: Run 100 dijkstras
    //run_problem_4(&graph);

    // Problem 5: Read input file and generate output
    run_problem_5(&graph);
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

    let path = "core/data/graphs/germany.fmi";
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

fn run_problem_4(graph: &Graph) {
    println!("Running problem 4: Running 100 dijkstras");

    let start = Instant::now();

    let n = graph.num_nodes;
    let mut rng = thread_rng();

    for _ in 0..100 {
        let source = rng.gen_range(0..n);
        let target = rng.gen_range(0..n);

        graph.dijkstra_distance(source, target);
    }

    let duration = start.elapsed();
    let average_dijkstra_duration = duration / 100;

    println!("Problem 4 time: {:?}. Average dijkstra duration: {:?}", duration, average_dijkstra_duration);
}

fn read_input_file(path: &str) -> Vec<(usize, usize)> {
    let file = File::open(path).expect("Cannot open input file");
    let reader = BufReader::new(file);

    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }

        let source = parts[0].parse::<usize>().unwrap();
        let target = parts[1].parse::<usize>().unwrap();

        pairs.push((source, target));
    }

    pairs
}

fn write_output_file(path: &str, lines: &[String]) {
    let mut file = File::create(path).expect("Cannot create output file");

    for line in lines {
        writeln!(file, "{}", line).expect("Write failed");
    }
}

fn run_problem_5(graph: &Graph) {
    println!("Running problem 5: Run dijkstras on given nodes from input file and generate output");

    let source_target_pairs = read_input_file("core/data/input_problem5.txt");
    let mut output_lines: Vec<String> = Vec::new();

    for (source, target) in source_target_pairs {

        let start = Instant::now();

        let dist = graph.dijkstra_distance(source, target);

        let duration = start.elapsed().as_secs_f32();

        let line = format!("{} {} {} {}", source, target, dist, duration);
        output_lines.push(line);

    }

    write_output_file("core/data/output_problem5.txt", &output_lines);

}