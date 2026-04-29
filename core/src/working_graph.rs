use crate::graph::Graph;

#[derive(Debug, Clone)]
pub struct WorkingGraph {
    pub n: usize,

    pub outgoing: Vec<Vec<Edge>>,
    pub incoming: Vec<Vec<Edge>>,

    pub contracted: Vec<bool>,
    pub levels: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub target: usize,
    pub weight: u64,

    // CH-specific
    pub is_shortcut: bool,
    pub middle: Option<usize>,
}

impl WorkingGraph {
    pub fn new(
        n: usize,
        outgoing: Vec<Vec<Edge>>,
        incoming: Vec<Vec<Edge>>,
        levels: Vec<u64>,
    ) -> Self {
        Self {
            n,
            outgoing,
            incoming,
            contracted: vec![false; n],
            levels,
        }
    }

    pub fn outgoing(&self, u: usize) -> &Vec<Edge> {
        &self.outgoing[u]
    }

    pub fn incoming(&self, u: usize) -> &Vec<Edge> {
        &self.incoming[u]
    }

    pub fn is_contracted(&self, u: usize) -> bool {
        self.contracted[u]
    }

    pub fn contract(&mut self, u: usize) {
        self.contracted[u] = true;
    }

    pub fn set_level(&mut self, u: usize, level: u64) {
        self.levels[u] = level;
    }

    pub fn level(&self, u: usize) -> u64 {
        self.levels[u]
    }
}

impl WorkingGraph {
    pub fn add_shortcut(&mut self, from: usize, to: usize, weight: u64, middle: usize) {
        let edge = Edge {
            target: to,
            weight,
            is_shortcut: true,
            middle: Some(middle),
        };

        self.outgoing[from].push(edge.clone());
        self.incoming[to].push(edge);
    }
}

pub struct CHBuilder {
    pub g: WorkingGraph,
    pub current_level: u64,
}

impl CHBuilder {
    pub fn from_graph(graph: &Graph) -> Self {
        let n = graph.num_nodes;

        let mut outgoing = vec![vec![]; n];
        let mut incoming = vec![vec![]; n];

        for u in 0..n {
            for e in graph.outgoing(u) {
                outgoing[u].push(Edge {
                    target: e.target as usize,
                    weight: e.weight,
                    is_shortcut: false,
                    middle: None,
                });
            }

            for e in graph.incoming(u) {
                incoming[u].push(Edge {
                    target: e.target as usize,
                    weight: e.weight,
                    is_shortcut: false,
                    middle: None,
                });
            }
        }

        let levels = graph.levels.clone();

        Self {
            g: WorkingGraph::new(n, outgoing, incoming, levels),
            current_level: 0,
        }
    }

    pub fn run(&mut self) {
        while let Some(u) = self.pick_node() {
            self.contract_node(u);
        }
    }

    pub fn pick_node(&self) -> Option<usize> {
        (0..self.g.n).find(|&u| !self.g.contracted[u])
    }

    fn witness_search(&self, source: usize, target: usize, blocked: usize, limit: u64) -> bool {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = self.g.n;
        let mut dist = vec![u64::MAX; n];
        let mut heap = BinaryHeap::new();

        dist[source] = 0;
        heap.push((Reverse(0), source));

        while let Some((Reverse(d), u)) = heap.pop() {
            if u == target {
                return d <= limit;
            }

            if d > dist[u] || u == blocked {
                continue;
            }

            for e in &self.g.outgoing[u] {
                let v = e.target;
                if self.g.contracted[v] {
                    continue;
                }

                let nd = d + e.weight;

                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push((Reverse(nd), v));
                }
            }
        }

        false
    }

    pub fn contract_node(&mut self, u: usize) {
        let incoming = self.g.incoming[u].clone();
        let outgoing = self.g.outgoing[u].clone();

        for e1 in incoming {
            for e2 in &outgoing {
                let v = e1.target;
                let w = e2.target;

                if self.g.contracted[v] || self.g.contracted[w] {
                    continue;
                }

                let via_cost = e1.weight + e2.weight;

                let has_path = self.witness_search(v as usize, w as usize, u, via_cost);

                if !has_path {
                    self.g.outgoing[v as usize].push(Edge {
                        target: w,
                        weight: via_cost,
                        is_shortcut: true,
                        middle: Some(u),
                    });

                    self.g.incoming[w as usize].push(Edge {
                        target: v,
                        weight: via_cost,
                        is_shortcut: true,
                        middle: Some(u),
                    });
                }
            }
        }

        self.g.contracted[u] = true;
        self.g.levels[u] = self.current_level;
        self.current_level += 1;
    }
}
