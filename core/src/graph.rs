#[derive(Debug)]
pub struct Graph {
    pub num_nodes: usize,

    // Graph representation of outgoing edges
    pub outgoing_offsets: Vec<u64>,
    pub outgoing_edges: Vec<Edge>,

    // Incoming edges
    pub incoming_offsets: Vec<u64>,
    pub incoming_edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub target: u64,
    pub weight: u64,
}

impl Graph {
    pub fn new(
        num_nodes: usize,
        outgoing: (Vec<Edge>, Vec<u64>),
        incoming: (Vec<Edge>, Vec<u64>),
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
        let start = self.outgoing_offsets[node] as usize;
        let end = self.outgoing_offsets[node + 1] as usize;
        &&self.outgoing_edges[start..end]
    }

    pub fn incoming(&self, node: usize) -> &[Edge] {
        let start = self.incoming_offsets[node] as usize;
        let end = self.incoming_offsets[node + 1] as usize;
        &&self.incoming_edges[start..end]
    }
}