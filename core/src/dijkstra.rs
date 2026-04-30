use crate::ch::BitSet;
use crate::graph::Graph;
use std::cmp::Ordering;
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Distance {
    pub weight: usize,
    pub id: usize,
}

impl Distance {
    pub fn new(weight: usize, id: usize) -> Self {
        Distance { weight, id }
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip this to convert the defaulty max heap to a min heap
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra<'a> {
    graph: &'a Graph,

    weights: Vec<Option<usize>>,
    heap: BinaryHeap<Distance>,
    visited: Vec<usize>,

    old_start: Option<usize>,
    optimized: BitSet,
}

impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Dijkstra {
            graph,
            weights: vec![None; graph.num_nodes()],
            heap: BinaryHeap::new(),
            visited: Vec::new(),
            old_start: None,
            optimized: BitSet::new(graph.num_nodes()),
        }
    }

    fn reset(&mut self, s: usize, t: usize) -> Option<usize> {
        let mut reset = true;

        if let Some(old_start) = self.old_start {
            if old_start == s {
                reset = false;
                if self.optimized.get(t) {
                    return self.weights[t as usize];
                }
            }
        }

        self.old_start = Some(s);

        // Cleanup of previous run
        if reset {
            while let Some(node) = self.visited.pop() {
                self.weights[node as usize] = None;
                self.optimized.set(node, false);
            }
            self.heap.clear();

            // Push start to heap and set dist to 0
            self.heap.push(Distance::new(0, s));
            self.weights[s as usize] = Some(0);
            self.visited.push(s);
        }

        return None;
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

    // Normal dijkstra was way too slow so this monstrosity had to be created
    pub fn shortest_path_consider_contraction(
        &mut self,
        source: usize,
        target: usize,
        contracted: &BitSet,
    ) -> Option<usize> {
        if let Some(weight) = self.reset(source, target) {
            return Some(weight);
        }

        while let Some(Distance { weight, id }) = self.heap.pop() {
            if self.optimized.get(id) {
                continue;
            }
            self.optimized.set(id, true);

            for edge in self.graph.outgoing_edges(id) {
                if contracted.get(edge.target as usize) {
                    continue;
                }

                let v = edge.target as usize;
                let w = edge.weight as usize;

                if self.weights[v].is_none_or(|curr| weight + w < curr) {
                    self.weights[v] = Some(weight + w);
                    self.heap.push(Distance::new(weight + w, v));
                    self.visited.push(v);
                }
            }

            if id == target {
                return Some(weight);
            }
        }

        return None;
    }
}
