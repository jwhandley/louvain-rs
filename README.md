# Implementation of the Louvain community detection algorithm in Rust

This is an implementation of the Louvain community detection algorithm in Rust. It is largely based on the [networkx](https://networkx.org/documentation/stable/_modules/networkx/algorithms/community/louvain.html#louvain_partitions) implementation (with some deviations from the provided code to account for differences between the graph module in this crate and networkx). Currently it only works for directed graphs (as it was originally intended to be used in comparison with the [travel to work area detection algorithm](https://github.com/jwhandley/ttwa-detection)), but I aim to extend it to work with undirected graphs in the future.

# Performance

Because of the similarities between this implementation and the networkx one, it is natural to compare the performance between the two. I ran each one on the commute matrix data from the 2021 England and Wales Census 50 times and took the average:

| Implementation | Average time (ms) |
| -------------- | ----------------- |
| networkx       | 11085             |
| Rust           | 604               |

Overall the Rust implementation is about 20 times faster (not including the extra time spent on I/O in Python)