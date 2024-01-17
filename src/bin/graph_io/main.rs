use std::fs::File;
use std::io::{self, BufRead, Write};

mod parse_edge_txt;
mod reduce_graph;
use parse_edge_txt::parse_string;
use reduce_graph::store_val_2;

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

fn main() -> io::Result<()> {
    // Specify the path to the text file
    let input_file_path = "cal.cedge.txt";
    let output_file_path = "cal.edge";

    // Open the file
    let input_file = File::open(input_file_path)?;
    //let output_file = File::create(output_file_path)?;

    store_val_2(input_file, output_file_path)
}
