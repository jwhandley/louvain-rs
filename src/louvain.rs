use rand::seq::SliceRandom;

use crate::graph::{Graph, NodeIndex};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type CommunityIndex = usize;
pub type Community = HashSet<NodeIndex>;
pub type Partition = Vec<Community>;

pub fn calculate_modularity(graph: &Graph, partition: &Partition, resolution: f64) -> f64 {
    let mut q = 0.0;
    let m: f64 = graph.size();
    let out_degrees = graph.out_degrees();
    let in_degrees = graph.in_degrees();

    for community in partition.iter() {
        let mut sigma_tot = 0.0;
        let mut sigma_in = 0.0;

        for node in community.iter() {
            sigma_tot += out_degrees[*node] + in_degrees[*node];
            graph.out_edges(*node).for_each(|edge| {
                if community.contains(&edge.target) {
                    sigma_in += edge.weight;
                }
            });
            graph.in_edges(*node).for_each(|edge| {
                if community.contains(&edge.source) {
                    sigma_in += edge.weight;
                }
            })
        }

        q += sigma_in / (2.0 * m) - resolution * (sigma_tot / (2.0 * m)).powi(2);
    }
    q
}

fn gen_graph(graph: &Graph, partition: &Partition) -> Graph {
    let mut new_graph = Graph::new();
    for idx in 0..partition.len() {
        new_graph.add_node(idx, &idx.to_string());
    }

    let mut node2com = HashMap::default();
    for (idx, com) in partition.iter().enumerate() {
        for node in com {
            node2com.insert(node, idx);
        }
    }
    let mut tmp_edges = HashMap::default();
    for edge in graph.edges() {
        let source = node2com[&edge.source];
        let target = node2com[&edge.target];
        *tmp_edges.entry((source, target)).or_insert(0.0) += edge.weight;
    }

    for ((source, target), weight) in tmp_edges {
        new_graph.add_edge(source, target, weight);
    }

    new_graph
}

fn merge_partition(old_partition: &Partition, new_partition: &Partition) -> Partition {
    // Nodes in new partition correspond to communities in old partition
    // We need to merge the communities in the old partition to match the new partition

    let mut to_merge: HashMap<CommunityIndex, Community> = HashMap::default();

    for (idx, community) in new_partition.iter().enumerate() {
        for old_community in community {
            to_merge
                .entry(idx)
                .or_default()
                .extend(old_partition[*old_community].clone());
        }
    }

    to_merge.values().cloned().collect()
}

fn neighbor_weights(
    nbrs: &HashMap<usize, f64>,
    node2com: &HashMap<NodeIndex, CommunityIndex>,
) -> HashMap<CommunityIndex, f64> {
    let mut weights2com = HashMap::default();
    for (nbr, &wt) in nbrs.iter() {
        *weights2com.entry(node2com[nbr]).or_insert(0.0) += wt;
    }
    weights2com
}

fn optimize_modularity(graph: &Graph, resolution: f64) -> Partition {
    let mut node2com: HashMap<NodeIndex, usize> = HashMap::default();
    let mut new_partition = Vec::with_capacity(graph.node_count());
    for node in graph.node_indices() {
        node2com.insert(node, node);
        let mut community = Community::default();
        community.insert(node);
        new_partition.push(community);
    }

    let mut nbrs: Vec<HashMap<NodeIndex, f64>> = Vec::new();
    for node in graph.node_indices() {
        let mut nbr = HashMap::default();
        for edge in graph.out_edges(node) {
            *nbr.entry(edge.target).or_insert(0.0) += edge.weight;
        }

        for edge in graph.in_edges(node) {
            *nbr.entry(edge.source).or_insert(0.0) += edge.weight;
        }
        nbr.entry(node).or_insert(0.0);
        nbrs.push(nbr);
    }

    let in_degrees = graph.in_degrees();
    let out_degrees = graph.out_degrees();

    let mut sigma_tot_in = in_degrees.clone();
    let mut sigma_tot_out = out_degrees.clone();
    let m = graph.size();

    let mut nodes = graph.node_indices().collect::<Vec<_>>();
    nodes.shuffle(&mut rand::thread_rng());

    let mut n_moves = 1;
    while n_moves > 0 {
        n_moves = 0;

        for node in nodes.iter() {
            let initial_com = node2com[node];
            let mut best_com = initial_com;
            let mut best_delta = 0.0;

            let mut weights2com = neighbor_weights(&nbrs[*node], &node2com);
            weights2com.entry(initial_com).or_insert(0.0);

            let in_degree = in_degrees[*node];
            let out_degree = out_degrees[*node];

            sigma_tot_in[initial_com] -= in_degree;
            sigma_tot_out[initial_com] -= out_degree;

            let remove_cost = -weights2com[&initial_com] / m
                + resolution
                    * (out_degree * sigma_tot_in[initial_com]
                        + in_degree * sigma_tot_out[initial_com])
                    / (m * m);

            for (&com, &wt) in weights2com.iter() {
                let delta = remove_cost + wt / m
                    - resolution
                        * (out_degree * sigma_tot_in[com] + in_degree * sigma_tot_out[com])
                        / (m * m);

                if delta > best_delta {
                    best_delta = delta;
                    best_com = com;
                }
            }

            sigma_tot_in[best_com] += in_degree;
            sigma_tot_out[best_com] += out_degree;

            if best_com != initial_com {
                n_moves += 1;
                new_partition[initial_com].remove(node);
                new_partition[best_com].insert(*node);
                node2com.insert(*node, best_com);
            }
        }
    }

    new_partition.retain(|com| !com.is_empty());

    new_partition
}

pub fn louvain_communities(graph: &Graph, resolution: f64, threshold: f64) -> Partition {
    let mut partition = optimize_modularity(graph, resolution);

    let mut modularity = calculate_modularity(graph, &partition, resolution);

    let mut improvement = true;

    while improvement {
        improvement = false;

        let new_graph = gen_graph(graph, &partition);

        let mut new_partition = optimize_modularity(&new_graph, resolution);
        new_partition = merge_partition(&partition, &new_partition);

        let new_modularity = calculate_modularity(graph, &new_partition, resolution);

        if new_modularity - modularity > threshold {
            improvement = true;
            modularity = new_modularity;
            partition = new_partition;
        }
    }

    partition
}
