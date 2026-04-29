use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug)]
pub struct Node {
    osm_id: u64,
    lat: f32,
    lon: f32,
    pub level: u16,
}

impl Node {
    pub fn new(osm_id: u64, lat: f32, lon: f32, level: u16) -> Self {
        Node {
            osm_id,
            lat,
            lon,
            level,
        }
    }

    pub fn set_level(&mut self, level: u16) {
        self.level = level
    }
}

static EDGE_COUNTER: OnceLock<AtomicI32> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub target: u64,
    pub weight: u64,

    pub edge_id_a: Option<u64>,
    pub edge_id_b: Option<u64>,

    id: i32,
    dir: bool,
}

impl Edge {
    pub fn new(target: u64, weight: u64, edge_id_a: Option<u64>, edge_id_b: Option<u64>) -> Self {
        let counter = EDGE_COUNTER.get_or_init(|| AtomicI32::new(0));
        counter.fetch_add(1, Ordering::Relaxed);

        Edge {
            target,
            weight,
            edge_id_a,
            edge_id_b,
            id: Self::get_creation_count(),
            dir: true,
        }
    }

    fn get_creation_count() -> i32 {
        EDGE_COUNTER
            .get()
            .map(|c| c.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    pub fn reverse(&self, source: u64) -> Self {
        let mut edge = self.clone();
        edge.dir = false;
        edge.target = source;
        return edge;
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Vec<Edge>>) -> Self {
        Graph { nodes, edges }
    }

    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).expect("File not found");
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();

        let num_nodes: usize = lines[0].parse().unwrap();
        let num_edges: usize = lines[1].parse().unwrap();

        let mut nodes: Vec<Node> = Vec::with_capacity(num_nodes);
        for i in 0..num_nodes {
            let line = &lines[i + 2];
            let parts: Vec<&str> = line.split_whitespace().collect();

            let osm_id: u64 = parts[0].parse().unwrap();
            let lat: f32 = parts[1].parse().unwrap();
            let lon: f32 = parts[2].parse().unwrap();
            let mut level = 0;

            // This is an already CH precomputed graph file
            if parts.len() >= 6 {
                level = parts[5].parse().unwrap();
            }

            nodes.push(Node::new(osm_id, lat, lon, level));
        }

        let mut edges: Vec<Vec<Edge>> = vec![Vec::new(); num_nodes];
        let edge_start = 2 + num_nodes;
        for i in 0..num_edges {
            let line = &lines[edge_start + i];
            let parts: Vec<&str> = line.split_whitespace().collect();

            let source: usize = parts[0].parse().unwrap();
            let target: usize = parts[1].parse().unwrap();
            let weight: u64 = parts[2].parse().unwrap();

            let edge_id_a: Option<u64> = parts[5].parse().ok();
            let edge_id_b: Option<u64> = parts[6].parse().ok();

            let edge: Edge = Edge::new(target as u64, weight, edge_id_a, edge_id_b);
            let reverse_edge = edge.reverse(source as u64);
            edges[source].push(edge);
            edges[target].push(reverse_edge);
        }

        return Graph { nodes, edges };
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges
            .iter()
            .map(|edge| edge.iter().filter(|&edge| edge.dir).count())
            .sum()
    }

    pub fn add_edge(&mut self, source: usize, edge: Edge) {
        let reverse_edge = edge.reverse(source as u64);

        if let Some(e) = self.edges[source as usize]
            .iter_mut()
            .find(|e| e.target == edge.target && e.dir == edge.dir)
        {
            if e.weight > edge.weight {
                e.weight = edge.weight;
            }
        } else {
            self.edges[source as usize].push(edge);
        }

        if let Some(e) = self.edges[edge.target as usize]
            .iter_mut()
            .find(|e| e.target == reverse_edge.target && e.dir == reverse_edge.dir)
        {
            if e.weight > reverse_edge.weight {
                e.weight = reverse_edge.weight;
            }
        } else {
            self.edges[edge.target as usize].push(reverse_edge);
        }
    }

    pub fn node_at(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    pub fn outgoing_edges(&self, node: usize) -> impl Iterator<Item = &Edge> {
        self.edges[node].iter().filter(|e| e.dir)
    }

    pub fn incoming_edges(&self, node: usize) -> impl Iterator<Item = &Edge> {
        self.edges[node].iter().filter(|e| !e.dir)
    }

    pub fn ch_query(&self, source: usize, target: usize) -> Option<usize> {
        let n = self.num_nodes();

        // Distances for forwards and backwards search
        let mut dist_f = vec![usize::MAX; n];
        let mut dist_b = vec![usize::MAX; n];

        let mut visited_f = vec![false; n];
        let mut visited_b = vec![false; n];

        let mut heap_f = BinaryHeap::new();
        let mut heap_b = BinaryHeap::new();

        dist_f[source] = 0;
        dist_b[target] = 0;

        heap_f.push((Reverse(0), source));
        heap_b.push((Reverse(0), target));

        let mut best = usize::MAX;

        while !heap_f.is_empty() || !heap_b.is_empty() {
            // ---- Forward search ----
            if let Some((Reverse(d), u)) = heap_f.pop() {
                if d > dist_f[u] || d > best {
                    continue;
                }

                visited_f[u] = true;

                if visited_b[u] {
                    best = best.min(dist_f[u] + dist_b[u]);
                }

                let level_u = self.node_at(u).level;

                for edge in self.outgoing_edges(u) {
                    let v = edge.target as usize;

                    // CH constraint: only go UP
                    if self.node_at(v).level <= level_u {
                        continue;
                    }

                    let new_dist = d + edge.weight as usize;

                    if new_dist < dist_f[v] {
                        dist_f[v] = new_dist;
                        heap_f.push((Reverse(new_dist), v));
                    }
                }
            }

            // ---- Backward search ----
            if let Some((Reverse(d), u)) = heap_b.pop() {
                if d > dist_b[u] || d > best {
                    continue;
                }

                visited_b[u] = true;

                if visited_f[u] {
                    best = best.min(dist_f[u] + dist_b[u]);
                }

                let level_u = self.node_at(u).level;

                for edge in self.incoming_edges(u) {
                    let v = edge.target as usize;

                    // CH constraint: only go UP
                    if self.node_at(v).level <= level_u {
                        continue;
                    }

                    let new_dist = d + edge.weight as usize;

                    if new_dist < dist_b[v] {
                        dist_b[v] = new_dist;
                        heap_b.push((Reverse(new_dist), v));
                    }
                }
            }
        }

        if best == usize::MAX { None } else { Some(best) }
    }

    pub fn ch_query_with_sod(&self, source: usize, target: usize) -> Option<usize> {
        let n = self.num_nodes();

        // Distances for forwards and backwards search
        let mut dist_f = vec![usize::MAX; n];
        let mut dist_b = vec![usize::MAX; n];

        let mut visited_f = vec![false; n];
        let mut visited_b = vec![false; n];

        let mut heap_f = BinaryHeap::new();
        let mut heap_b = BinaryHeap::new();

        dist_f[source] = 0;
        dist_b[target] = 0;

        heap_f.push((Reverse(0), source));
        heap_b.push((Reverse(0), target));

        let mut best = usize::MAX;

        while !heap_f.is_empty() || !heap_b.is_empty() {
            // =========================
            // FORWARD SEARCH
            // =========================
            if let Some((Reverse(d), u)) = heap_f.pop() {
                if d > dist_f[u] || d > best {
                    continue;
                }

                // ---- Stall-on-Demand ----
                let mut stalled = false;
                for edge in self.incoming_edges(u) {
                    let v = edge.target as usize;
                    let w = edge.weight as usize;

                    if self.node_at(v).level <= self.node_at(u).level {
                        continue;
                    }

                    if let Some(vd) = dist_f[v].checked_add(w) {
                        if vd < dist_f[u] {
                            stalled = true;
                            break;
                        }
                    }
                }
                if stalled {
                    continue;
                }

                visited_f[u] = true;

                if visited_b[u] {
                    best = best.min(dist_f[u] + dist_b[u]);
                }

                let level_u = self.node_at(u).level;

                for edge in self.outgoing_edges(u) {
                    let v = edge.target as usize;

                    // CH constraint: only upward edges
                    if self.node_at(v).level <= level_u {
                        continue;
                    }

                    let new_dist = d + edge.weight as usize;

                    if new_dist < dist_f[v] {
                        dist_f[v] = new_dist;
                        heap_f.push((Reverse(new_dist), v));
                    }
                }
            }

            // =========================
            // BACKWARD SEARCH
            // =========================
            if let Some((Reverse(d), u)) = heap_b.pop() {
                if d > dist_b[u] || d > best {
                    continue;
                }

                // ---- Stall-on-Demand ----
                let mut stalled = false;
                for edge in self.outgoing_edges(u) {
                    let v = edge.target as usize;
                    let w = edge.weight as usize;

                    if self.node_at(v).level <= self.node_at(u).level {
                        continue;
                    }

                    if let Some(vd) = dist_b[v].checked_add(w) {
                        if vd < dist_b[u] {
                            stalled = true;
                            break;
                        }
                    }
                }
                if stalled {
                    continue;
                }

                visited_b[u] = true;

                if visited_f[u] {
                    best = best.min(dist_f[u] + dist_b[u]);
                }

                let level_u = self.node_at(u).level;

                for edge in self.incoming_edges(u) {
                    let v = edge.target as usize;

                    // CH constraint: only upward edges
                    if self.node_at(v).level <= level_u {
                        continue;
                    }

                    let new_dist = d + edge.weight as usize;

                    if new_dist < dist_b[v] {
                        dist_b[v] = new_dist;
                        heap_b.push((Reverse(new_dist), v));
                    }
                }
            }
        }

        if best == usize::MAX { None } else { Some(best) }
    }
}
