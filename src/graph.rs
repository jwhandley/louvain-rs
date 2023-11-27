#![allow(dead_code)]
use std::rc::Rc;

pub type NodeIndex = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
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
    edges: Vec<Rc<Edge>>,
    pub in_adjacency: Vec<Vec<Rc<Edge>>>,
    pub out_adjacency: Vec<Vec<Rc<Edge>>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            in_adjacency: Vec::new(),
            out_adjacency: Vec::new(),
        }
    }

    pub fn size(&self) -> f64 {
        self.out_degrees().iter().sum()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Rc<Edge>> {
        self.edges.iter()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_weights(&self) -> impl Iterator<Item = f64> + '_ {
        self.in_adjacency.iter().flatten().map(|e| e.weight)
    }

    pub fn add_node(&mut self, node_index: NodeIndex, id: &str) {
        let node = Node {
            index: node_index,
            id: String::from(id),
            in_degree: 0.0,
            out_degree: 0.0,
        };
        self.in_adjacency.push(Vec::new());
        self.out_adjacency.push(Vec::new());
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, weight: f64) {
        let edge = Rc::new(Edge {
            source,
            target,
            weight,
        });
        self.nodes[edge.source].out_degree += edge.weight;
        self.nodes[edge.target].in_degree += edge.weight;
        self.out_adjacency[edge.source].push(edge.clone());
        self.in_adjacency[edge.target].push(edge.clone());
        self.edges.push(edge);
    }

    pub fn in_edges(&self, node: NodeIndex) -> impl Iterator<Item = &Rc<Edge>> + '_ {
        self.in_adjacency[node].iter()
    }

    pub fn out_edges(&self, node: NodeIndex) -> impl Iterator<Item = &Rc<Edge>> + '_ {
        self.out_adjacency[node].iter()
    }

    pub fn in_degrees(&self) -> Vec<f64> {
        self.nodes.iter().map(|n| n.in_degree).collect()
    }

    pub fn out_degrees(&self) -> Vec<f64> {
        self.nodes.iter().map(|n| n.out_degree).collect()
    }

    pub fn neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.out_edges(node)
            .map(|e| e.target)
            .chain(self.in_edges(node).map(|e| e.source))
    }

    pub fn edges_connecting(
        &self,
        source: NodeIndex,
        target: NodeIndex,
    ) -> impl Iterator<Item = &Rc<Edge>> {
        self.out_edges(source)
            .filter(move |&e| e.target == target)
            .chain(self.in_edges(source).filter(move |&e| e.source == target))
    }

    pub fn node_indices(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.nodes.iter().map(|n| n.index)
    }
}
