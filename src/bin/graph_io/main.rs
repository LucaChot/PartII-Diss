mod parse_edge_txt;
mod reduce_graph;
mod types;
use reduce_graph::complete_reduction;


fn main() {
    // Specify the path to the text file
    let input_file_path = "cal.cedge.txt";
    let output_edges = "cal.edge";
    let output_nodes = "cal.nodes";
    let output_chains = "cal.chains";

    // Open the file
    complete_reduction(input_file_path, output_edges, output_nodes, output_chains)
}
