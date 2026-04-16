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
}