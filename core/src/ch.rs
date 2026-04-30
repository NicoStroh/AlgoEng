use std::cmp::Reverse;
use std::collections::BinaryHeap;

use rayon::prelude::*;

use crate::dijkstra::Dijkstra;
use crate::graph::{Edge, Graph};

struct Shortcut {
    from: u64,
    to: u64,
    weight: u64,
    edge_id_a: u64,
    edge_id_b: u64,
}

impl Shortcut {
    pub fn new(from: u64, to: u64, weight: u64, edge_id_a: u64, edge_id_b: u64) -> Self {
        Shortcut {
            from,
            to,
            weight,
            edge_id_a,
            edge_id_b,
        }
    }
}

pub struct CH<'a> {
    graph: &'a mut Graph,

    dist_forward: Vec<Option<usize>>,
    dist_backward: Vec<Option<usize>>,

    visited_forward: Vec<usize>,
    visited_backward: Vec<usize>,

    heap_forward: BinaryHeap<(Reverse<usize>, usize)>,
    heap_backward: BinaryHeap<(Reverse<usize>, usize)>,
}

impl<'a> CH<'a> {
    pub fn new(graph: &'a mut Graph) -> Self {
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

    fn calc_shortcuts(
        &self,
        dijkstra: &mut Dijkstra,
        node: usize,
        contracted: &BitSet,
    ) -> Vec<Shortcut> {
        let mut shortcuts: Vec<Shortcut> = Vec::new();

        for incoming_edge in self.graph.incoming_edges(node) {
            if contracted.get(incoming_edge.target as usize) {
                continue;
            }

            for outgoing_edge in self.graph.outgoing_edges(node) {
                if contracted.get(outgoing_edge.target as usize) {
                    continue;
                }

                let direct_distance = incoming_edge.weight + outgoing_edge.weight;
                let shortest_distance = dijkstra.shortest_path_consider_contraction(
                    incoming_edge.target as usize,
                    outgoing_edge.target as usize,
                    contracted,
                );
                if shortest_distance >= Some(direct_distance as usize) {
                    shortcuts.push(Shortcut::new(
                        incoming_edge.target,
                        outgoing_edge.target,
                        direct_distance,
                        incoming_edge.id as u64,
                        outgoing_edge.id as u64,
                    ));
                }
            }
        }

        shortcuts
    }

    fn find_independent_set(
        &self,
        nodes: &mut Vec<usize>,
        contracted: &BitSet,
    ) -> Vec<(isize, usize)> {
        let mut independent_set: Vec<(isize, usize)> = Vec::new();

        // Represents nodes that must not be chosen in this iteration
        let mut blocked = BitSet::new(self.graph.num_nodes() as usize);
        // Those nodes are contracted later
        let mut blocked_nodes = Vec::new();

        for node in nodes.iter().map(|node| *node) {
            if blocked.get(node as usize) {
                blocked_nodes.push(node);
                continue;
            }

            let incoming_num = self
                .graph
                .incoming_edges(node)
                .filter(|edge| !contracted.get(edge.target as usize))
                .count() as isize;
            let outgoing_num = self
                .graph
                .outgoing_edges(node)
                .filter(|edge| !contracted.get(edge.target as usize))
                .count() as isize;

            independent_set.push((
                incoming_num * outgoing_num - incoming_num - outgoing_num,
                node,
            ));

            // Block its neighbors from being selected
            for edge in self.graph.edges[node].clone() {
                blocked.set(edge.target as usize, true);
            }
        }

        independent_set.sort_by_key(|x| x.0);
        // Index of first element with edge diff > 0
        let zero_or_less = independent_set.partition_point(|x| x.0 <= 0);

        // Only take subset with low edge diff. n is the limit between nodes that are chosen and those who are blocked
        let n = if zero_or_less > 0 {
            zero_or_less.max(independent_set.len().div_ceil(16))
        } else {
            independent_set.len().div_ceil(8)
        };

        // Add the nodes, that were left out in this iteration to blocked nodes
        for (_, node) in &independent_set[n..] {
            blocked_nodes.push(*node);
        }

        independent_set.truncate(n);
        *nodes = blocked_nodes;
        return independent_set;
    }

    fn contract_independent_set(
        &mut self,
        independent_set: &Vec<(isize, usize)>,
        level: u16,
        contracted: &mut BitSet,
    ) {
        // Calculate shortcuts in parallel
        let chunk_size = independent_set.len().div_ceil(rayon::current_num_threads());
        let results: Vec<(usize, Vec<Shortcut>)> = independent_set
            .par_chunks(chunk_size)
            .flat_map(|chunk| {
                let mut dijkstra = Dijkstra::new(&self.graph);
                chunk
                    .iter()
                    .map(|&(_, node)| {
                        let shortcuts = self.calc_shortcuts(&mut dijkstra, node, contracted);
                        (node, shortcuts)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Add shortcuts to the graph and update node levels
        for (node, shortcuts) in results {
            for Shortcut {
                from,
                to,
                weight,
                edge_id_a,
                edge_id_b,
            } in shortcuts
            {
                self.graph.add_edge(
                    from as usize,
                    Edge::new(to, weight, Some(edge_id_a), Some(edge_id_b)),
                );
            }

            self.graph.node_at_mut(node).set_level(level);
            contracted.set(node, true);
        }
    }

    pub fn ch_preprocess(&mut self) {
        let mut contracted = BitSet::new(self.graph.num_nodes() as usize);
        let mut nodes: Vec<usize> = { (0..self.graph.num_nodes() as usize).collect() };

        for level in 0.. {
            let independent_set = self.find_independent_set(&mut nodes, &contracted);
            if independent_set.is_empty() {
                break;
            }
            self.contract_independent_set(&independent_set, level, &mut contracted);
        }
    }

    pub fn ch_query(&mut self, source: usize, target: usize) -> Option<usize> {
        self.cleanup();

        self.dist_forward[source] = Some(0);
        self.heap_forward.push((Reverse(0), source));
        self.visited_forward.push(source);

        self.dist_backward[target] = Some(0);
        self.heap_backward.push((Reverse(0), target));
        self.visited_backward.push(target);

        let mut best = usize::MAX;

        while !self.heap_forward.is_empty() || !self.heap_backward.is_empty() {
            // =========================
            // FORWARD SEARCH
            // =========================
            if let Some((Reverse(d), u)) = self.heap_forward.pop() {
                if d > self.dist_forward[u].unwrap_or(usize::MAX) || d > best {
                    continue;
                }

                // ---- Stall-on-Demand ----
                let mut stalled = false;
                for edge in self.graph.incoming_edges(u) {
                    let v = edge.target as usize;
                    let w = edge.weight as usize;

                    if self.graph.node_at(v).level <= self.graph.node_at(u).level {
                        continue;
                    }

                    if let Some(vd) = self.dist_forward[v].unwrap_or(usize::MAX).checked_add(w) {
                        if vd < self.dist_forward[u].unwrap_or(usize::MAX) {
                            stalled = true;
                            break;
                        }
                    }
                }
                if stalled {
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

                    // CH constraint: only upward edges
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

            // =========================
            // BACKWARD SEARCH
            // =========================
            if let Some((Reverse(d), u)) = self.heap_backward.pop() {
                if d > self.dist_backward[u].unwrap_or(usize::MAX) || d > best {
                    continue;
                }

                // ---- Stall-on-Demand ----
                let mut stalled = false;
                for edge in self.graph.outgoing_edges(u) {
                    let v = edge.target as usize;
                    let w = edge.weight as usize;

                    if self.graph.node_at(v).level <= self.graph.node_at(u).level {
                        continue;
                    }

                    if let Some(vd) = self.dist_backward[v].unwrap_or(usize::MAX).checked_add(w) {
                        if vd < self.dist_backward[u].unwrap_or(usize::MAX) {
                            stalled = true;
                            break;
                        }
                    }
                }
                if stalled {
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

                    // CH constraint: only upward edges
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

pub struct BitSet {
    data: Vec<u64>,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size.div_ceil(64)],
        }
    }

    // Reurns true if the bit at idx is set, false otherwise
    pub fn get(&self, idx: usize) -> bool {
        (self.data[idx / 64] >> (idx % 64)) & 1 != 0
    }

    // Sets the bit at idx to value
    pub fn set(&mut self, idx: usize, value: bool) {
        if value {
            self.data[idx / 64] |= 1 << (idx % 64);
        } else {
            self.data[idx / 64] &= !(1 << (idx % 64));
        }
    }

    pub fn clear(&mut self, idx: usize) {
        self.data[idx / 64] &= !(1 << (idx % 64));
    }
}
