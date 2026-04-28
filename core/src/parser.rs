use std::fs::File;
use std::io::{BufRead, BufReader};

type OffsetArray = (Vec<Edge>, Vec<Node>);

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

use crate::graph::{Graph, Edge, Node};

pub fn parse_graph(path: &str) -> Graph {
    let lines = read_lines(path);

    let num_nodes: usize = lines[0].parse().unwrap();
    let num_edges: usize = lines[1].parse().unwrap();

    // =========================
    // 1. LEVELS EINLESEN
    // =========================
    let mut levels: Vec<u64> = vec![];

    for i in 0..num_nodes {
        let line = &lines[i + 2];
        let parts: Vec<&str> = line.split_whitespace().collect();

        let level: u64 = parts[5].parse().unwrap();
        levels.push(level);
    }

    // =========================
    // 2. SORTIERUNG NACH LEVEL
    // =========================
    let mut order: Vec<usize> = (0..num_nodes).collect();

    order.sort_by_key(|&i| levels[i]);

    let mut new_id = vec![0usize; num_nodes];
    let mut sorted_levels = vec![0u64; num_nodes];

    for (i, &old) in order.iter().enumerate() {
        new_id[old] = i;
        sorted_levels[i] = levels[old];
    }

    // =========================
    // 3. EDGE LISTE MIT REMAP
    // =========================
    let mut outgoing_edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];
    let mut incoming_edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];

    let edge_start = 2 + num_nodes;

    for i in 0..num_edges {
        let line = &lines[edge_start + i];
        let parts: Vec<&str> = line.split_whitespace().collect();

        let source: usize = parts[0].parse().unwrap();
        let target: usize = parts[1].parse().unwrap();
        let weight: u64 = parts[2].parse().unwrap();

        let edge_id_a: Option<u64> = parts[5].parse().ok();
        let edge_id_b: Option<u64> = parts[6].parse().ok();

        let s = new_id[source];
        let t = new_id[target];

        outgoing_edges[s].push(Edge::new(t as u64, weight, edge_id_a, edge_id_b));
        incoming_edges[t].push(Edge::new(s as u64, weight, edge_id_b, edge_id_a));
    }

    // =========================
    // 4. OFFSET ARRAY BAUEN
    // =========================
    let outgoing = create_offset_array(outgoing_edges);
    let incoming = create_offset_array(incoming_edges);

    Graph::new(
        num_nodes,
        sorted_levels,
        outgoing,
        incoming
    )
}

fn create_offset_array(adj_list: Vec<Vec<Edge>>) -> OffsetArray {

    let mut flat_edges = Vec::new();
    let mut nodes: Vec<Node> = Vec::with_capacity(adj_list.len() + 1);

    let mut current_offset = 0u64;

    nodes.push(Node::new(0));

    for edges in adj_list.iter() {
        current_offset += edges.len() as u64;
        flat_edges.extend(edges.clone());
        nodes.push(Node::new(current_offset));
    }

    (flat_edges, nodes)
}