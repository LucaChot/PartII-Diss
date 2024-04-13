use serde::{Serialize,Deserialize};
use std::fs::File;
use std::io::{self, Write};
use std::cell::RefCell;
use std::rc::Rc;

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


#[derive(Serialize, Deserialize, Debug)]
pub enum ReducedState {
  INCHAIN(ChainState),
  INGRAPH(usize)
}

#[derive(Debug)]
pub struct Node{
  pub rc_edge : Vec<Rc<Edge>>,
  pub visited : RefCell<bool>,
  pub reduced_state : ReducedState
}

impl Node {
  pub fn new (index : usize)  -> Node {
    Node { rc_edge : Vec::new(), visited : RefCell::new(false), reduced_state : ReducedState::INGRAPH(index) }
  }

  pub fn is_val_2 (&self) -> bool {
   self.rc_edge.len() == 2 
  }

  pub fn add_edge(&mut self, edge : &Rc<Edge>) {
    self.rc_edge.push(Rc::clone(edge));
  }
  
  pub fn store_nodes(nodes : &Vec<Node>, output_file_path : &str) -> io::Result<()> {
    let mut file = File::create(output_file_path)?;
    for node in nodes {
      let serialized = bincode::serialize(&node.reduced_state).unwrap();
      file.write_all(&serialized)?;
    }
    Ok(())
  }

  pub fn get_reduced(nodes : &Vec<Node>) -> Vec<&ReducedState> {
    nodes.iter().map(|node| &node.reduced_state).collect::<Vec<_>>()
  }
}

#[derive(Debug)]
pub struct Edge {
    pub node_a : usize,
    pub node_b : usize,
    pub distance : f64,
    pub visited: RefCell<bool>,
    pub order: RefCell<usize>,
}

impl Edge {
    pub fn new(node_a : usize, node_b : usize, distance : f64) -> Self {
        Edge { node_a,
          node_b,
          distance,
          visited : RefCell::new(false),
          order :  RefCell::new(0)
        }
    }

  pub fn is_connected(&self, b : &Edge) -> Option<(bool,bool)>{ 
    match self.node_a {
      n if n == b.node_a => Some((true, true)),
      n if n == b.node_b => Some((true, false)),
      _ => match self.node_b {
        n if n == b.node_a => Some((false, true)),
        n if n == b.node_b => Some((false, false)),
        _ => None,
      },
    }
  }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.node_a == other.node_a 
          && self.node_b == other.node_b 
          && self.distance == other.distance
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      match (self.node_a.cmp(&other.node_a),self.node_b.cmp(&other.node_b)) {
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => 
          Some(self.distance.total_cmp(&other.distance)),
        _ => None
      }
    }
}

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

  pub fn get_edge(&self) -> Rc<Edge>{
    let mut ends = [self.start, self.end];
    ends.sort();
    Rc::new(Edge::new(ends[0], ends[1], self.dist))
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
