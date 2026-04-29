use crate::ch::BitSet;
use crate::graph::Graph;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Dijkstra<'a> {
    graph: &'a Graph,
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Dijkstra { graph }
    }

    pub fn dijkstra(&self, source: usize, target: usize) -> usize {
        let n = self.graph.num_nodes();

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

            for edge in self.graph.outgoing_edges(u) {
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

    pub fn shortest_path_consider_contraction(
        &mut self,
        source: usize,
        target: usize,
        contracted: &BitSet,
    ) -> usize {
        let n = self.graph.num_nodes();

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

            for edge in self.graph.outgoing_edges(u) {
                if contracted.get(edge.target as usize) {
                    continue;
                }

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
}
