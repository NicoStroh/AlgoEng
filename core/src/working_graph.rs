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
