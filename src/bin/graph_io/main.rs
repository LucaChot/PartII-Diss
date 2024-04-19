mod reduce_graph;
mod adj_matrix;
mod solve;
mod types;
use reduce_graph::complete_reduction;


fn main() {
    // Specify the path to the text file
    let input_file_path = "SF.cedge.txt";
    let output_edges = "SF.edge";
    let output_nodes = "SF.nodes";
    let output_chains = "SF.chains";

    // Open the file
    complete_reduction(input_file_path, output_edges, output_nodes, output_chains)
}
