use std::fs::{File, self};
use std::io::{self, BufRead};
use crate::parse_edge_txt::parse_string;

#[derive(Clone,Copy)]
struct NodeState {
  state : u8
}

impl NodeState {
  fn new ()  -> NodeState {
    NodeState { state : 0 }
  }

  fn is_val_2 (&self) -> bool {
   self.state == 2 
  }

  fn add_link(&mut self) {
    match self.state {
      3 => (),
      num => self.state = num + 1,
    }
  }
}

pub fn store_val_2(input_file : File, mut output_file_path : &str) -> io::Result<()> {
  const NUM_NODES : usize = 21048;
  let mut nodes_status = [NodeState::new(); NUM_NODES];
  let input_reader = io::BufReader::new(input_file);
  // Iterate over the lines in the file
  for line in input_reader.lines() {
      // Handle each line
      match line {
          Ok(content) => {
            match parse_string(&content) {
              Ok((start_node, end_node, _)) => { 
                nodes_status[start_node].add_link(); 
                nodes_status[end_node].add_link(); 
              },
              Err(err) => eprintln!("Error reading line: {}", err),
            }
          }
          Err(err) => eprintln!("Error reading line: {}", err),
      }
  }
  let formatted_strings : Vec<String> = nodes_status.iter().enumerate().map(|(node, &node_state)| {
    format!("{} {}", node, node_state.is_val_2())
  }).collect();
  let count = nodes_status.iter().filter(|&&node_state| 
    node_state.is_val_2()
  ).count();
  println!("{}", count);
  let result = formatted_strings.join("\n");

  fs::write(output_file_path, result)
}
