#![allow(dead_code)]
use crate::graph::Graph;
use crate::louvain::Partition;
use std::path::Path;
use anyhow::Result;

pub fn read_pajek(path: &Path) -> Graph {
    // Example format:
    //*Vertices 34
    // 1 "v1"
    // 2 "v2"
    // 3 "v3"
    // 4 "v4"
    //*Arcs
    // 2 1 1.000000000000000
    // 3 1 1.000000000000000
    // 4 1 1.000000000000000
    let file = std::fs::read_to_string(path).unwrap();

    let mut graph = Graph::new();

    let mut lines = file.lines();
    let mut line = lines.next().unwrap();
    while !line.starts_with("*Vertices") {
        line = lines.next().unwrap();
    }

    let num_vertices = line
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    for i in 0..num_vertices {
        lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let id = parts.next().unwrap();

        graph.add_node(i, id);
    }

    while !line.starts_with("*Arcs") {
        line = lines.next().unwrap();
    }

    for line in lines {
        let mut parts = line.split_whitespace();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let target = parts.next().unwrap().parse::<usize>().unwrap();
        let weight = parts.next().unwrap().parse::<f64>().unwrap();

        graph.add_edge(source - 1, target - 1, weight);
    }

    graph
}

pub fn read_adjacency_matrix(path: &Path) -> Graph {
    // Example format:
    // lad11cd,E41000001,E41000002,E41000003,E41000004
    // E41000001,20777,1591,534,3865,
    // E41000002,990,25474,5111,8889,
    // E41000003,558,10569,25106,5387,
    // E41000004,2590,10779,3652,44337,
    // E41000005,266,1300,467,3014,
    // E41000047,2665,1656,592,4176,
    // E41000048,59,60,34,187,
    // E41000276,111,151,44,214,
    // E41000277,137,211,68,290,
    // E41000278,31,102,43,170,
    // E41000279,115,111,58,190,
    // E41000280,399,279,114,654,

    // Read csv file
    let mut rdr = csv::Reader::from_path(path).unwrap();

    // Get the header, the first row and the first column are IDs which we ignore
    let header = rdr.headers().unwrap();
    let header = header.iter().skip(1).collect::<Vec<_>>();

    // Create a new graph
    let mut graph = Graph::new();

    // Add nodes to the graph
    for (i, item) in header.iter().enumerate() {
        graph.add_node(i, item);
    }

    // Add edges to the graph
    for (i, row) in rdr.records().enumerate() {
        let row = row.unwrap();
        for (j, cell) in row.iter().skip(1).enumerate() {
            let weight = cell.parse::<f64>().unwrap();
            if weight > 0.0 {
                graph.add_edge(i, j, weight);
            }
        }
    }

    graph
}

pub fn write_partition_to_csv(partition: &Partition, graph: &Graph, path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(["id","area"])?;

    for (idx, community) in partition.iter().enumerate() {
        for node in community.iter() {
            writer.write_record([
                graph.nodes[*node].id.as_str(),
                idx.to_string().as_str()
            ])?;
        }
    }
    Ok(())
}