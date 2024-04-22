mod reduce_graph;
mod adj_matrix;
mod solve;
mod types;
use std::io;

use reduce_graph::complete_reduction;


fn main() -> io::Result<()> {
    // Specify the path to the text file
    let input_file_path = "OL.cedge.txt";
    let output_edges = "OL.edge";
    let output_nodes = "OL.nodes";
    let output_chains = "OL.chains";

    // Open the file
    complete_reduction(input_file_path, output_edges, output_nodes, output_chains)?;
    Ok(())
}
