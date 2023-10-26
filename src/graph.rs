#![allow(dead_code)]
use std::collections::HashMap;

pub type NodeIndex = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    id: String,
    pub index: NodeIndex,
    pub in_degree: f64,
    pub out_degree: f64,
}

impl Node {
    pub fn new(index: NodeIndex, id: &str) -> Node {
        Node {
            index,
            id: String::from(id),
            in_degree: 0.0,
            out_degree: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub weight: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub nodes: Vec<Node>,
    edges: Vec<Edge>,
    pub in_adjacency: HashMap<NodeIndex, Vec<Edge>>,
    pub out_adjacency: HashMap<NodeIndex, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            in_adjacency: HashMap::new(),
            out_adjacency: HashMap::new(),
        }
    }

    pub fn size(&self) -> f64 {
        self.out_degrees().iter().sum()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_weights(&self) -> impl Iterator<Item = f64> + '_ {
        self.in_adjacency.values().flatten().map(|e| e.weight)
    }

    pub fn add_node(&mut self, node_index: NodeIndex, id: &str) {
        let node = Node {
            index: node_index,
            id: String::from(id),
            in_degree: 0.0,
            out_degree: 0.0,
        };
        self.in_adjacency.entry(node_index).or_default();
        self.out_adjacency.entry(node_index).or_default();
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, weight: f64) {
        let edge = Edge {
            source,
            target,
            weight,
        };
        self.nodes[edge.source].out_degree += edge.weight;
        self.nodes[edge.target].in_degree += edge.weight;
        self.out_adjacency
            .entry(edge.source)
            .or_default()
            .push(edge.clone());
        self.in_adjacency
            .entry(edge.target)
            .or_default()
            .push(edge.clone());
        self.edges.push(edge);
    }

    pub fn in_edges(&self, node: NodeIndex) -> &Vec<Edge> {
        &self.in_adjacency[&node]
    }

    pub fn out_edges(&self, node: NodeIndex) -> &Vec<Edge> {
        &self.out_adjacency[&node]
    }

    pub fn in_degrees(&self) -> Vec<f64> {
        self.nodes.iter().map(|n| n.in_degree).collect()
    }

    pub fn out_degrees(&self) -> Vec<f64> {
        self.nodes.iter().map(|n| n.out_degree).collect()
    }

    pub fn neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.out_edges(node)
            .iter()
            .map(|e| e.target)
            .chain(self.in_edges(node).iter().map(|e| e.source))
    }

    pub fn edges_connecting(
        &self,
        source: NodeIndex,
        target: NodeIndex,
    ) -> impl Iterator<Item = &Edge> {
        self.out_edges(source)
            .iter()
            .filter(move |&e| e.target == target)
            .chain(
                self.in_edges(source)
                    .iter()
                    .filter(move |&e| e.source == target),
            )
    }

    pub fn node_indices(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.nodes.iter().map(|n| n.index)
    }
}
