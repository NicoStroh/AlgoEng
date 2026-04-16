#[derive(Debug)]
pub struct Graph {
    pub num_nodes: usize,

    // Für ausgehende Kanten
    pub offsets: Vec<usize>,
    pub edges: Vec<Edge>,

    // Für eingehende Kanten
    pub rev_offsets: Vec<usize>,
    pub rev_edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub to: usize,
    pub weight: u32,
}

impl Graph {
    pub fn neighbors(&self, node: usize) -> &[Edge] {
        let start = self.offsets[node];
        let end = self.offsets[node + 1];
        &self.edges[start..end]
    }

    pub fn incoming(&self, node: usize) -> &[Edge] {
        let start = self.rev_offsets[node];
        let end = self.rev_offsets[node + 1];
        &self.rev_edges[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let graph = Graph {
            num_nodes: 2,
            offsets: vec![0, 1, 1],
            edges: vec![Edge { to: 1, weight: 5 }],
            rev_offsets: vec![0, 0, 1],
            rev_edges: vec![Edge { to: 0, weight: 5 }],
        };

        let n = graph.neighbors(0);
        assert_eq!(n[0].to, 1);
        assert_eq!(n[0].weight, 5);
    }
}