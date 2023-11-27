use std::path::PathBuf;

mod graph;
mod io;
mod louvain;

fn main() {
    let path = PathBuf::from("data/ttwa-matrix.csv");
    let graph = io::read_adjacency_matrix(&path);
    // let path = PathBuf::from("data/karate.net");
    // let graph = io::read_pajek(&path);

    // Start perfcount
    let start = std::time::Instant::now();
    let partition = louvain::louvain_communities(&graph, 1.0, 1e-7);

    // println!("Communities: {}", partition.len());
    // println!(
    //     "Modularity: {}",
    //     louvain::calculate_modularity(&graph, &partition, 1.0)
    // );
    // println!("Finished in {}ms", start.elapsed().as_millis());
    println!("{}", start.elapsed().as_millis());

    let save_path = PathBuf::from("output/msoa-communities.csv");
    io::write_partition_to_csv(&partition, &graph, &save_path)
        .expect("Should have been able to write to directory.");
}
