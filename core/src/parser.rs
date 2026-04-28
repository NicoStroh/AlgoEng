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

    let mut levels: Vec<u64> = vec![];
    for i in 0..num_nodes {
        // add 2 because of first 2 lines indicating number of nodes and edges
        let line = &lines[i + 2];

        let parts: Vec<&str> = line.split_whitespace().collect();

        let id: u64 = parts[0].parse().unwrap();
        let level: u64 = parts[5].parse().unwrap();
        levels.push(level);
    }

    let mut outgoing_edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];
    let mut incoming_edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];

    // Skip nodes
    let edge_start = 2 + num_nodes;

    for i in 0..num_edges {
        let line = &lines[edge_start + i];
        let parts: Vec<&str> = line.split_whitespace().collect();

        let source: u64 = parts[0].parse().unwrap();
        let target: u64 = parts[1].parse().unwrap();
        let weight: u64 = parts[2].parse().unwrap();

        let edge_id_a: Option<u64> = parts[5].parse().ok();
        let edge_id_b: Option<u64> = parts[6].parse().ok();

        outgoing_edges[source as usize].push(Edge::new(target, weight, edge_id_a, edge_id_b));
        incoming_edges[target as usize].push(Edge::new(source, weight, edge_id_b, edge_id_a));
    }

    let outgoing = create_offset_array(outgoing_edges, &levels);
    let incoming = create_offset_array(incoming_edges, &levels);

    Graph::new(
        num_nodes,
        outgoing,
        incoming
    )

}

fn create_offset_array(adj_list: Vec<Vec<Edge>>, levels: &Vec<u64>) -> OffsetArray {

    let mut flat_edges = Vec::new();
    let mut nodes: Vec<Node> = Vec::with_capacity(adj_list.len() + 1);
    let mut current_offset = 0u64;

    for (i, edges) in adj_list.iter().enumerate() {
        current_offset += edges.len() as u64;        
        flat_edges.extend(edges.clone());
        nodes.push(Node::new(current_offset, levels[i]));
    }

    (flat_edges, nodes)

}