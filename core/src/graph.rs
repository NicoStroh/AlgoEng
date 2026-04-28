
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Graph {
    pub num_nodes: usize,

    // Graph representation of outgoing edges
    pub outgoing_nodes: Vec<Node>,
    pub outgoing_edges: Vec<Edge>,

    // Incoming edges
    pub incoming_nodes: Vec<Node>,
    pub incoming_edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub target: u64,
    pub weight: u64,
    edge_id_a: Option<u64>,
    edge_id_b: Option<u64>
}

impl Edge {
    pub fn new(target: u64, weight: u64, edge_id_a: Option<u64>, edge_id_b: Option<u64>) -> Self {
        Edge {target, weight, edge_id_a, edge_id_b}
    }
}

#[derive(Debug, Clone)]
 pub struct Node {
    offset: u64,
    level: u64
 }

 impl Node {
    pub fn new(offset: u64, level: u64) -> Self {
        Node {offset, level}
    }
 }

impl Graph {
    pub fn new(
        num_nodes: usize,
        outgoing: (Vec<Edge>, Vec<Node>),
        incoming: (Vec<Edge>, Vec<Node>),
    ) -> Self {
        let (outgoing_edges, outgoing_nodes) = outgoing;
        let (incoming_edges, incoming_nodes) = incoming;

        Self {
            num_nodes,
            outgoing_nodes,
            outgoing_edges,
            incoming_nodes,
            incoming_edges,
        }
    }

    pub fn outgoing(&self, node: usize) -> &[Edge] {
        let start = self.outgoing_nodes[node].offset as usize;
        let end = self.outgoing_nodes[node + 1].offset as usize;
        &&self.outgoing_edges[start..end]
    }

    pub fn incoming(&self, node: usize) -> &[Edge] {
        let start = self.incoming_nodes[node].offset as usize;
        let end = self.incoming_nodes[node + 1].offset as usize;
        &&self.incoming_edges[start..end]
    }

    // Only computes the difference between source and target node
    pub fn dijkstra_distance(&self, source: usize, target: usize) -> usize {
        let n = self.num_nodes;

        let mut dist = vec![usize::MAX; n];
        let mut prev = vec![None; n];

        let mut heap = BinaryHeap::new();

        dist[source] = 0;
        heap.push((Reverse(0), source));

        while let Some((Reverse(d), u)) = heap.pop() {
            if u == target {
                break;
            }

            if d > dist[u] {
                continue;
            }

            for edge in self.outgoing(u) {
                let v = edge.target as usize;
                let w = edge.weight as usize;

                let new_dist = d + w;

                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    prev[v] = Some(u);
                    heap.push((Reverse(new_dist), v));
                }
            }
        }

        return dist[target];
    }
    
    // Full dijkstra algorithm from source to target, returns the complete path
    pub fn dijkstra_path(&self, source: usize, target: usize,) -> Option<(usize, Vec<usize>)> {
        let n = self.num_nodes;

        let mut dist = vec![usize::MAX; n];
        let mut prev = vec![None; n];

        let mut heap = BinaryHeap::new();

        dist[source] = 0;
        heap.push((Reverse(0), source));

        while let Some((Reverse(d), u)) = heap.pop() {
            if u == target {
                break;
            }

            if d > dist[u] {
                continue;
            }

            for edge in self.outgoing(u) {
                let v = edge.target as usize;
                let w = edge.weight as usize;

                let new_dist = d + w;

                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    prev[v] = Some(u);
                    heap.push((Reverse(new_dist), v));
                }
            }
        }

        if dist[target] == usize::MAX {
            return None;
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut cur = target;

        while let Some(p) = prev[cur] {
            path.push(cur);
            cur = p;
        }

        path.push(source);
        path.reverse();

        Some((dist[target], path))
    }

}