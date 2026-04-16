use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Datei nicht gefunden");
    let reader = BufReader::new(file);

    reader
        .lines()
        .skip(5)
        .map(|l| l.unwrap())
        .collect()
}

use crate::graph::{Graph, Edge};

pub fn parse_graph(path: &str) -> Graph {
    let lines = read_lines(path);

    let num_nodes: usize = lines[0].parse().unwrap();
    let num_edges: usize = lines[1].parse().unwrap();

    // temporäre Speicherung
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];
    let mut rev_adj: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];

    // Nodes überspringen (interessieren uns erstmal nicht)
    let edge_start = 2 + num_nodes;

    for i in 0..num_edges {
        let line = &lines[edge_start + i];
        let parts: Vec<&str> = line.split_whitespace().collect();

        let from: usize = parts[0].parse().unwrap();
        let to: usize = parts[1].parse().unwrap();
        let weight: u32 = parts[2].parse().unwrap();

        adj[from].push(Edge { to, weight });
        rev_adj[to].push(Edge { to: from, weight });
    }

    build_graph(num_nodes, adj, rev_adj)
}

fn build_graph(
    num_nodes: usize,
    adj: Vec<Vec<Edge>>,
    rev_adj: Vec<Vec<Edge>>,
) -> Graph {
    let mut offsets = Vec::with_capacity(num_nodes + 1);
    let mut edges = Vec::new();

    offsets.push(0);

    for neighbors in &adj {
        edges.extend(neighbors);
        offsets.push(edges.len());
    }

    let mut rev_offsets = Vec::with_capacity(num_nodes + 1);
    let mut rev_edges = Vec::new();

    rev_offsets.push(0);

    for neighbors in &rev_adj {
        rev_edges.extend(neighbors);
        rev_offsets.push(rev_edges.len());
    }

    Graph {
        num_nodes,
        offsets,
        edges,
        rev_offsets,
        rev_edges,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mv() {
        let g = parse_graph("/Users/nicostrohbach/AlgoEng/graphs/MV.fmi");

        assert_eq!(g.num_nodes, 644199);
        assert_eq!(g.neighbors(0)[0].to, 434859);
    }

    #[test]
    fn test_parse_germany() {
        let g = parse_graph("/Users/nicostrohbach/AlgoEng/graphs/germany.fmi");

        assert_eq!(g.num_nodes, 25115477);
        assert_eq!(g.neighbors(0)[0].to, 1488520);
    }
}