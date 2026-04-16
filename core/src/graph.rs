
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Graph {
    pub num_nodes: usize,

    // Graph representation of outgoing edges
    pub outgoing_offsets: Vec<usize>,
    pub outgoing_edges: Vec<Edge>,

    // Incoming edges
    pub incoming_offsets: Vec<usize>,
    pub incoming_edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub target: usize,
    pub weight: usize,
}

impl Graph {
    pub fn new(
        num_nodes: usize,
        outgoing: (Vec<Edge>, Vec<usize>),
        incoming: (Vec<Edge>, Vec<usize>),
    ) -> Self {
        let (outgoing_edges, outgoing_offsets) = outgoing;
        let (incoming_edges, incoming_offsets) = incoming;

        Self {
            num_nodes,
            outgoing_offsets,
            outgoing_edges,
            incoming_offsets,
            incoming_edges,
        }
    }

    pub fn outgoing(&self, node: usize) -> &[Edge] {
        let start = self.outgoing_offsets[node];
        let end = self.outgoing_offsets[node + 1];
        &&self.outgoing_edges[start..end]
    }

    pub fn incoming(&self, node: usize) -> &[Edge] {
        let start = self.incoming_offsets[node];
        let end = self.incoming_offsets[node + 1];
        &&self.incoming_edges[start..end]
    }
    
    pub fn dijkstra(&self, source: usize, target: usize) -> Option<(usize, Vec<usize>)> {
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
                let v = edge.target;
                let w = edge.weight;

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