use std::fs::File;
use std::io::{BufRead, BufReader};

type OffsetArray = (Vec<Edge>, Vec<usize>);

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

use crate::graph::{Graph, Edge};

pub fn parse_graph(path: &str) -> Graph {
    let lines = read_lines(path);

    let num_nodes: usize = lines[0].parse().unwrap();
    let num_edges: usize = lines[1].parse().unwrap();

    let mut outgoing_edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];
    let mut incoming_edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];

    // Skip nodes
    let edge_start = 2 + num_nodes;

    for i in 0..num_edges {
        let line = &lines[edge_start + i];
        let parts: Vec<&str> = line.split_whitespace().collect();

        let source: usize = parts[0].parse().unwrap();
        let target: usize = parts[1].parse().unwrap();
        let weight: usize = parts[2].parse().unwrap();

        outgoing_edges[source].push(Edge { target, weight });
        incoming_edges[target].push(Edge { target: source, weight });
    }

    build_graph(num_nodes, outgoing_edges, incoming_edges)
}

fn create_offset_array(adj_list: Vec<Vec<Edge>>) -> OffsetArray {

    let mut edges = Vec::new();
    let mut offsets = Vec::with_capacity(adj_list.len() + 1);

    offsets.push(0);

    for neighbors in &adj_list {
        edges.extend(neighbors);
        offsets.push(edges.len());
    }

    (edges, offsets)

}

fn build_graph(
    num_nodes: usize,
    outgoing_edges: Vec<Vec<Edge>>,
    incoming_edges: Vec<Vec<Edge>>,
) -> Graph {

    // Create offset array for the outgoing edges
    let outgoing = create_offset_array(outgoing_edges);

    // Create offset array for incoming edges
    let incoming = create_offset_array(incoming_edges);

    Graph::new(
        num_nodes,
        outgoing,
        incoming
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mv() {
        let g = parse_graph("/Users/nicostrohbach/AlgoEng/graphs/MV.fmi");

        assert_eq!(g.num_nodes, 644199);
        assert_eq!(g.outgoing(0)[0].target, 434859);
    }

    #[test]
    fn test_parse_germany() {
        let g = parse_graph("/Users/nicostrohbach/AlgoEng/graphs/germany.fmi");

        assert_eq!(g.num_nodes, 25115477);
        assert_eq!(g.outgoing(0)[0].target, 1488520);
    }
}