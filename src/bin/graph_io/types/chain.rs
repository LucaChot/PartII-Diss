use serde::{Serialize,Deserialize};
use std::fs::File;
use std::io::{self, Write, Seek};
use std::rc::Rc;
use super::edge::Edge;

#[derive(Debug,Serialize, Deserialize)]
pub struct Chain {
  pub start : usize,
  pub end : usize, 
  pub nodes : Vec<(usize, f64)>,
  pub dist : f64,
}

impl Chain {
  pub fn new() -> Self {
    Chain { start : 0, end : 0, nodes : Vec::new(), dist : 0.0 }
  }

  pub fn get_path(&self, node_a : usize, node_b : usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut rev = false;
    let mut in_path = false;
    for (node, _) in &self.nodes {
      if !in_path {
        if *node == node_a {
          in_path = true;
          rev = false;
        }
        if *node == node_b {
          in_path = true;
          rev = true;
        }
      }
      if in_path {
        path.push(*node)
      }
      if in_path {
        if !rev && *node == node_b {
          break
        }
        if rev && *node == node_a {
          path.reverse();
          break
        }
      }
    }
    return path
  }

  pub fn get_edge(&self, chain_id : usize) -> Rc<Edge>{
    let mut ends = [self.start, self.end];
    ends.sort();
    Rc::new(Edge::from_chain(ends[0], ends[1], self.dist, chain_id))
  }

  pub fn chain_to_io(self, id : usize) -> Vec<String> {
    let order = self.start < self.end;
    let len = self.nodes.len() - 2;

    let node_to_io = |node : usize, dist : f64| {
      let str : String;
      if order {
        str = format!("{} {} {} {} {} {}",
                          node, id,self.start, dist, self.end, self.dist - dist);
      } else {
        str = format!("{} {} {} {} {} {}",
                          node, id, self.end, self.dist - dist, self.start, dist);
      }
      str
    };
    self.nodes.iter().skip(1).take(len).map(|&(node, dist)| node_to_io(node,dist)).collect::<Vec<_>>()
  }

}
pub fn store_chains(chains : &Vec<Chain>, output_file_path : &str) -> io::Result<()> {
  let mut file = File::create(output_file_path)?;
  let mut num_node = 0;
  let mut offset = (chains.len() * std::mem::size_of::<u64>()) as u64;
  for chain in chains {
    let offset_offset = num_node * std::mem::size_of::<u64>() as u64;
    let _ = file.seek(io::SeekFrom::Start(offset_offset));

    let serialized_offset = bincode::serialize(&offset).unwrap();
    file.write_all(&serialized_offset)?;

    let _ = file.seek(io::SeekFrom::Start(offset));
    let serialized = bincode::serialize(&chain).unwrap();
    file.write_all(&serialized)?;
    
    offset += bincode::serialized_size(&chain).unwrap();
    num_node += 1;
  }
  Ok(())
}

