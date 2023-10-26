#![allow(dead_code)]
use crate::graph::Graph;
use std::path::Path;

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
    for i in 0..header.len() {
        graph.add_node(i, header[i]);
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

pub fn read_edgelist(path: &Path) -> Graph {
    // Example format
    // 1 2
    // 1 3
    // 1 4
    // 1 5
    // 1 6
    // 1 7
    // 1 8
    // 1 9
    // 2 18
    // 2 32
    // 2 13
    // 2 16
    // 2 10
    // 2 24
    // 2 33
    // 2 34
    // 2 22

    let file = std::fs::read_to_string(path).unwrap();

    let mut graph = Graph::new();

    for line in file.lines() {
        let mut parts = line.split_whitespace();
        let source = parts.next().unwrap().parse::<usize>().unwrap();
        let target = parts.next().unwrap().parse::<usize>().unwrap();

        if !graph.node_indices().collect::<Vec<_>>().contains(&source) {
            graph.add_node(source, &source.to_string());
        }

        if !graph.node_indices().collect::<Vec<_>>().contains(&target) {
            graph.add_node(target, &target.to_string());
        }

        // Graph is undirected, so add edges in both directions
        graph.add_edge(source - 1, target - 1, 1.0);
        // graph.add_edge(target - 1, source - 1, 1.0);
    }

    graph
}
