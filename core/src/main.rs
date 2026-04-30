use crate::ch::CH;
use crate::graph::Graph;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

mod ch;
mod dijkstra;
mod graph;

fn main() {
    // Reading graph
    let start_read = Instant::now();
    let path = "graph.fmi";
    let mut graph = Graph::from_file(path);
    let duration_read = start_read.elapsed();
    println!("Reading graph took {:?}", duration_read);
    let num_edges = graph.num_edges();

    // Construct CH
    let start_preprocess = Instant::now();
    let mut ch = CH::new(&mut graph);
    ch.ch_preprocess();
    let duration_preprocess = start_preprocess.elapsed();
    println!("Preprocessing graph took {:?}", duration_preprocess);
    let new_edges = ch.graph.num_edges();

    // Export CH graph
    let start_export = Instant::now();
    let export_path = "graph.ch";
    let _ = ch.graph.export_graph(export_path);
    let duration_export = start_export.elapsed();
    println!("Exporting CH graph took {:?}", duration_export);

    // Export log
    export_log(duration_preprocess, duration_export, new_edges - num_edges);

    // Query results
    let mut result = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("results.txt")
        .unwrap();

    let queries = parse_queries("queries.txt").unwrap_or_else(|e| {
        eprintln!("Failed to parse queries: {}", e);
        Vec::new()
    });
    for (s, t) in queries {
        let start = Instant::now();
        let distance = ch.ch_query(s, t).unwrap();
        let duration = start.elapsed().as_micros();
        println!(
            "CH query from {} to {} took {:?} with distance {}",
            s, t, duration, distance
        );
        writeln!(result, "{} {} {} {}", s, t, distance, duration).unwrap();
    }
}

fn export_log(
    duration_preprocess: std::time::Duration,
    duration_export: std::time::Duration,
    created_edges: usize,
) {
    let log_path = "log.txt";
    let mut log_file = std::fs::File::create(log_path).expect("Failed to create log file");
    use std::io::Write;
    writeln!(
        log_file,
        "Preprocessing graph took {:?}",
        duration_preprocess
    )
    .expect("Failed to write to log file");
    writeln!(log_file, "Exporting CH graph took {:?}", duration_export)
        .expect("Failed to write to log file");
    writeln!(log_file, "Created {} edges", created_edges).expect("Failed to write to log file");
}

fn parse_queries(path: &str) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return None;
            }

            let source: usize = parts[0].parse().ok()?;
            let target: usize = parts[1].parse().ok()?;

            return Some((source, target));
        })
        .collect())
}
