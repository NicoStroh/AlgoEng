use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
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

    pub id: i32,
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

            let edge_id_a: Option<u64> = parts.get(5).and_then(|s| s.parse().ok());
            let edge_id_b: Option<u64> = parts.get(6).and_then(|s| s.parse().ok());

            let edge: Edge = Edge::new(target as u64, weight, edge_id_a, edge_id_b);
            let reverse_edge = edge.reverse(source as u64);
            edges[source].push(edge);
            edges[target].push(reverse_edge);
        }

        return Graph { nodes, edges };
    }

    pub fn export_graph(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        // Comment header
        for _ in 0..9 {
            writeln!(file, "# ")?;
        }
        // One empty line
        writeln!(file, "# ")?;

        // Number of nodes and edges
        writeln!(file, "{}", self.num_nodes())?;
        writeln!(file, "{}", self.num_edges())?;

        // Export nodes
        for (i, node) in self.nodes.iter().enumerate() {
            // <ID> <OSMID> <Lat> <Lon> <Height> <Level>
            writeln!(
                file,
                "{} {} {} {} 0 {}",
                i, node.osm_id, node.lat, node.lon, node.level
            )?;
        }

        // Export edges
        for (node, edges) in self.edges.iter().enumerate() {
            for edge in edges {
                if !edge.dir {
                    continue;
                }

                // <SrcID> <TrgID> <Weight> <Type> <MaxSpeed> <EdgeIdA> <EdgeIdB>
                writeln!(
                    file,
                    "{} {} {} 0 0 {:?} {:?}",
                    node, edge.target, edge.weight, edge.edge_id_a, edge.edge_id_b
                )?;
            }
        }

        Ok(())
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

    pub fn node_at_mut(&mut self, index: usize) -> &mut Node {
        &mut self.nodes[index]
    }

    pub fn outgoing_edges(&self, node: usize) -> impl Iterator<Item = &Edge> {
        self.edges[node].iter().filter(|e| e.dir)
    }

    pub fn incoming_edges(&self, node: usize) -> impl Iterator<Item = &Edge> {
        self.edges[node].iter().filter(|e| !e.dir)
    }
}
