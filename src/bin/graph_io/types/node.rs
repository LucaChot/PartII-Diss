use serde::{Serialize,Deserialize};
use std::fs::File;
use std::io::{self, Write, Seek};
use std::cell::RefCell;
use std::rc::Rc;
use super::edge::Edge;


#[derive(Serialize, Deserialize,Debug)]
pub struct ChainState {
  pub chain_id : usize,
  pub end_a : usize,
  pub dist_a : f64,
  pub end_b : usize,
  pub dist_b : f64,
}

impl ChainState {
  pub fn new (chain_id : usize, end_a : usize, dist_a : f64, end_b : usize, dist_b : f64)  -> ChainState {
    ChainState { chain_id , end_a , dist_a , end_b, dist_b }
  }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct GraphState {
  pub reduced_id : usize,
  pub chains : Vec<usize>,
}

impl GraphState {
  pub fn new (reduced_id : usize)  -> Self {
    Self { reduced_id , chains : Vec::new()}
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReducedState {
  INCHAIN(ChainState),
  INGRAPH(GraphState)
}

pub fn store_states(nodes : &Vec<&ReducedState>, output_file_path : &str) -> io::Result<()> {
  let mut file = File::create(output_file_path)?;
  let mut num_node = 0;
  let mut offset = (nodes.len() * std::mem::size_of::<u64>()) as u64;
  for node in nodes {
    let offset_offset = num_node * std::mem::size_of::<u64>() as u64;
    let _ = file.seek(io::SeekFrom::Start(offset_offset));

    let serialized_offset = bincode::serialize(&offset).unwrap();
    file.write_all(&serialized_offset)?;

    let _ = file.seek(io::SeekFrom::Start(offset));
    let serialized = bincode::serialize(*node).unwrap();
    file.write_all(&serialized)?;
    
    offset += bincode::serialized_size(*node).unwrap();
    num_node += 1;
  }
  Ok(())
}

#[derive(Debug)]
pub struct Node{
  pub rc_edge : Vec<Rc<Edge>>,
  pub visited : RefCell<bool>,
  pub reduced_state : ReducedState
}

impl Node {
  pub fn new (index : usize)  -> Node {
    Node { rc_edge : Vec::new(), visited : RefCell::new(false),
    reduced_state : ReducedState::INGRAPH(GraphState::new(index)) }
  }

  pub fn is_val_2 (&self) -> bool {
   self.rc_edge.len() == 2 
  }

  pub fn add_edge(&mut self, edge : &Rc<Edge>) {
    self.rc_edge.push(Rc::clone(edge));
  }
  

  pub fn get_reduced(nodes : &Vec<Node>) -> Vec<&ReducedState> {
    nodes.iter().map(|node| &node.reduced_state).collect::<Vec<_>>()
  }
}
