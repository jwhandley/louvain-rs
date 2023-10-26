# Implementation of the Louvain community detection algorithm in Rust

This is an implementation of the Louvain community detection algorithm in Rust. It is largely based on the [networkx](https://networkx.org/documentation/stable/_modules/networkx/algorithms/community/louvain.html#louvain_partitions) implementation (with some deviations from the provided code to account for differences between the graph module in this crate and networkx). Currently it only works for directed graphs (as it was originally intended to be used in comparison with the [travel to work area detection algorithm](https://github.com/jwhandley/ttwa-detection)), but I aim to extend it to work with undirected graphs in the future.

# Performance

Because of the similarities between this implementation and the networkx one, it is natural to compare the performance between the two. The table below shows the speed of the algorithm for three example graphs:
1. The karate club graph with 34 nodes
2. The commute matrix from the 2011 UK Census (404 nodes, one per local authority district)
3. The commute matrix from the 2021 England and Wales Census (7201 nodes, one per middle supoer output layer)

The table below shows indicative running times for the algorithm using both networkx and this crate. This is only from a handful of tests, so is not authoritative. However, the Rust version appears to be about 10 times as fast as the Python version. This could most likely be improved greatly with further optimization, but is a showcase of how much faster even poorly optimized Rust code can be than native Python code.

| Graph               | Nodes | networkx | This crate |
| ------------------- | ----- | -------- | ---------- |
| Karate club         | 34    | 3ms      | 0ms        |
| 2011 commute matrix | 404   | 1100ms   | 110ms      |
| 2021 commute matrix | 7201  | 15000ms  | 1500ms     |