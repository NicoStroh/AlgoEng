use crate::graph::Graph;
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct CH<'a> {
    graph: &'a Graph,

    dist_forward: Vec<Option<usize>>,
    dist_backward: Vec<Option<usize>>,

    visited_forward: Vec<usize>,
    visited_backward: Vec<usize>,

    heap_forward: BinaryHeap<(Reverse<usize>, usize)>,
    heap_backward: BinaryHeap<(Reverse<usize>, usize)>,
}

impl<'a> CH<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let n = graph.num_nodes();

        let dist_forward = vec![None; n];
        let dist_backward = vec![None; n];

        let visited_forward = Vec::new();
        let visited_backward = Vec::new();

        let heap_forward = BinaryHeap::new();
        let heap_backward = BinaryHeap::new();
        CH {
            graph,
            dist_forward,
            dist_backward,
            visited_forward,
            visited_backward,
            heap_forward,
            heap_backward,
        }
    }

    pub fn cleanup(&mut self) {
        while let Some(node) = self.visited_forward.pop() {
            self.dist_forward[node as usize] = None;
            self.dist_backward[node as usize] = None;
        }
        while let Some(node) = self.visited_backward.pop() {
            self.dist_forward[node as usize] = None;
            self.dist_backward[node as usize] = None;
        }
        self.heap_forward.clear();
        self.heap_backward.clear();
    }

    pub fn ch_preprocess(&mut self) {
        // TODO
    }

    pub fn ch_query(&self, source: usize, target: usize) -> usize {
        // TODO
        return 0;
    }

    pub fn ch_query_without_sod(&mut self, source: usize, target: usize) -> Option<usize> {
        self.cleanup();

        self.dist_forward[source] = Some(0);
        self.heap_forward.push((Reverse(0), source));
        self.visited_forward.push(source);

        self.dist_backward[target] = Some(0);
        self.heap_backward.push((Reverse(0), target));
        self.visited_backward.push(target);

        let mut best = usize::MAX;

        while !self.heap_forward.is_empty() || !self.heap_backward.is_empty() {
            // ---- Forward search ----
            if let Some((Reverse(d), u)) = self.heap_forward.pop() {
                if d > self.dist_forward[u].unwrap_or(usize::MAX) || d > best {
                    continue;
                }

                self.visited_forward.push(u);

                if self.visited_backward.contains(&u) {
                    best = best.min(
                        self.dist_forward[u].unwrap_or(usize::MAX)
                            + self.dist_backward[u].unwrap_or(usize::MAX),
                    );
                }

                let level_u = self.graph.node_at(u).level;

                for edge in self.graph.outgoing_edges(u) {
                    let v = edge.target as usize;

                    // CH constraint: only go UP
                    if self.graph.node_at(v).level <= level_u {
                        continue;
                    }

                    let new_dist = d + edge.weight as usize;

                    if new_dist < self.dist_forward[v].unwrap_or(usize::MAX) {
                        self.dist_forward[v] = Some(new_dist);
                        self.heap_forward.push((Reverse(new_dist), v));
                    }
                }
            }

            // ---- Backward search ----
            if let Some((Reverse(d), u)) = self.heap_backward.pop() {
                if d > self.dist_backward[u].unwrap_or(usize::MAX) || d > best {
                    continue;
                }

                self.visited_backward.push(u);

                if self.visited_forward.contains(&u) {
                    best = best.min(
                        self.dist_forward[u].unwrap_or(usize::MAX)
                            + self.dist_backward[u].unwrap_or(usize::MAX),
                    );
                }

                let level_u = self.graph.node_at(u).level;

                for edge in self.graph.incoming_edges(u) {
                    let v = edge.target as usize;

                    // CH constraint: only go UP
                    if self.graph.node_at(v).level <= level_u {
                        continue;
                    }

                    let new_dist = d + edge.weight as usize;

                    if new_dist < self.dist_backward[v].unwrap_or(usize::MAX) {
                        self.dist_backward[v] = Some(new_dist);
                        self.heap_backward.push((Reverse(new_dist), v));
                    }
                }
            }
        }

        if best == usize::MAX { None } else { Some(best) }
    }
}
