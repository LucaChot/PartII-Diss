use std::fs::File;
use std::io::{self, BufRead, Write};

mod parse_edge_txt;
mod reduce_graph;
mod types;
use parse_edge_txt::parse_string;
use reduce_graph::complete_reduction;

fn create_bin_file(input_file : File, mut output_file : File) -> io::Result<()> {
  let input_reader = io::BufReader::new(input_file);

  // Iterate over the lines in the file
  for line in input_reader.lines() {
      // Handle each line
      match line {
          Ok(content) => {
            match parse_string(&content) {
              Ok((start_node, end_node, distance)) => { 
                output_file.write_all(&start_node.to_ne_bytes())?;
                output_file.write_all(&end_node.to_ne_bytes())?;
                output_file.write_all(&distance.to_ne_bytes())?;
              },
              Err(err) => eprintln!("Error reading line: {}", err),
            }
          }
          Err(err) => eprintln!("Error reading line: {}", err),
      }
  }

  Ok(())
}

fn main() -> io::Result<()>{
    // Specify the path to the text file
    let input_file_path = "cal.cedge.txt";
    let output_edges = "cal.edge";
    let output_nodes = "cal.nodes";

    // Open the file
    complete_reduction(input_file_path, output_edges, output_nodes)
}
