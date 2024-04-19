use std::{fs::File, io::{self, Seek}, ops::Add};
use sim::types::{Matrix, Msg};
use crate::types::chain::Chain;
use crate::types::node::ReducedState;

trait Intersect<T> {
  fn intersect(&mut self, other : &mut Self) -> Option<T>;
}

impl<T : Copy + Ord> Intersect<T> for Vec<T> {
  fn intersect(&mut self, other : &mut Self) -> Option<T> {
    self.sort();
    other.sort();
    let mut self_iter = self.iter();
    let mut other_iter = other.iter();
    let mut self_item = self_iter.next();
    let mut other_item = other_iter.next();
    loop {
      match (self_item, other_item) {
        (Some(&x), Some(&y)) => { 
          if x == y {
            return Some(x);
          }
          if x < y {
            self_item = self_iter.next()
          } else {
            other_item = other_iter.next()
          }
        }
        _ => {
          return None;
        }
      }
    }
  }
}


pub struct Path {
  distance : f64,
  nodes : Vec<usize>
}

impl Path {
  pub fn new(distance : f64, nodes : Vec<usize>) -> Self{
    Path {distance, nodes }
  }
}

impl<'a,'b> Add<&'b Path> for &'a Path {
  type Output = Path;

  fn add(self, other : &'b Path) -> Path {
    let total_distance = self.distance + other.distance;
    let mut path = self.nodes.clone();
    for &node in other.nodes.iter().skip(1) {
      path.push(node);
    }
    Path::new(total_distance, path)
  }
}
 impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

pub struct Solver {
  adj_matrix : Matrix<Msg>,
  nodes : File,
  chains : File,
}

impl Solver {
  pub fn new(adj_matrix : Matrix<Msg>, nodes_file : &str, chains_file : &str)
    -> io::Result<Self> {
    let nodes = File::open(nodes_file)?;
    let chains = File::open(chains_file)?;

    Ok(Solver { adj_matrix, nodes, chains })
    }
}

impl Solver {

  fn get_state(&mut self, node_id : usize) -> ReducedState{
    let offset_offset = (node_id * std::mem::size_of::<u64>()) as u64;

    let _ = self.nodes.seek(io::SeekFrom::Start(offset_offset));
    let offset : u64 = bincode::deserialize_from(&mut self.nodes).unwrap();

    let _ = self.nodes.seek(io::SeekFrom::Start(offset));
    bincode::deserialize_from(&mut self.nodes).unwrap()
  }

  fn get_chain(&mut self, chain_id : usize) -> Chain {
    let offset_offset = (chain_id * std::mem::size_of::<u64>()) as u64;

    let _ = self.chains.seek(io::SeekFrom::Start(offset_offset));
    let offset : u64 = bincode::deserialize_from(&mut self.chains).unwrap();

    let _ = self.chains.seek(io::SeekFrom::Start(offset));
    bincode::deserialize_from(&mut self.chains).unwrap()
  }

  fn path_along_edge(&mut self, node_a : usize, node_b : usize) -> Path {
    let state_a = self.get_state(node_a);
    let state_b = self.get_state(node_b);
    
    match(state_a, state_b) {
      (ReducedState::INGRAPH(mut adj_a), ReducedState::INGRAPH(mut adj_b)) => {
        let distance = self.adj_matrix[adj_a.reduced_id][adj_b.reduced_id].get_w();
        match adj_a.chains.intersect(&mut adj_b.chains) {
          None => 
            Path::new(distance, vec![adj_a.reduced_id, adj_b.reduced_id]),
          Some(chain_id) => {
            let chain = self.get_chain(chain_id);
            Path::new(distance, chain.get_path(node_a, node_b))
          }

        }
      },
      (ReducedState::INGRAPH(_), ReducedState::INCHAIN(inchain_b)) => {
        let distance = if node_a == inchain_b.end_a {
          inchain_b.dist_a
        } else {
          inchain_b.dist_b
        };
        let chain = self.get_chain(inchain_b.chain_id);
        Path::new(distance, chain.get_path(node_a, node_b))
      },
      (ReducedState::INCHAIN(_), ReducedState::INGRAPH(_)) => {
        let mut path = self.path_along_edge(node_b, node_a);
        path.nodes.reverse();
        path
      },
      (ReducedState::INCHAIN(inchain_a), ReducedState::INCHAIN(inchain_b)) => {
        if inchain_a.chain_id != inchain_b.chain_id {
          Path::new(-1.0, Vec::new())
        } else {
          let distance  = if inchain_a.dist_a >=  inchain_b.dist_a {
            inchain_a.dist_a - inchain_a.dist_a
          } else {
            inchain_b.dist_a - inchain_a.dist_a
          };
          let chain = self.get_chain(inchain_b.chain_id);
          Path::new(distance, chain.get_path(node_a, node_b))
        }
      },
    }
  }


  fn find_path(&mut self, node_a : usize, node_b : usize) -> Path {
    let state_a = self.get_state(node_a);
    let state_b = self.get_state(node_b);

    match (state_a, state_b) {
      (ReducedState::INGRAPH(adj_a), ReducedState::INGRAPH(adj_b)) => {
        let mut path = Path::new(0.0, Vec::new());
        let mut last = adj_b.reduced_id;
        let mut previous = self.adj_matrix[adj_a.reduced_id][adj_b.reduced_id].get_p();
        while previous != last {
          path = &self.path_along_edge(previous, last) + &path;
          last = previous;
          previous = self.adj_matrix[adj_a.reduced_id][adj_b.reduced_id].get_p();
        }
        path
      },
      (ReducedState::INCHAIN(inchain_a), ReducedState::INGRAPH(_)) => {
        let path1 = &self.path_along_edge(node_a, inchain_a.end_a) + 
          &self.find_path(inchain_a.end_a, node_b);
        let path2 = &self.path_along_edge(node_a, inchain_a.end_b) + 
          &self.find_path(inchain_a.end_b, node_b);
        if path1 < path2 {
          path1
        } else {
          path2
        }
      },
      (ReducedState::INGRAPH(_), ReducedState::INCHAIN(inchain_b)) => {
        let path1 = &self.find_path(node_a, inchain_b.end_a) + 
          &self.path_along_edge(inchain_b.end_a, node_b);
        let path2 = &self.find_path(node_a, inchain_b.end_b) + 
          &self.path_along_edge(inchain_b.end_b, node_b);
        if path1 < path2 {
          path1
        } else {
          path2
        }
      },
      (ReducedState::INCHAIN(inchain_a), ReducedState::INCHAIN(inchain_b)) => {
        if inchain_a.chain_id == inchain_b.chain_id {
          let path1 = self.path_along_edge(node_a, node_b);
          let path2 = &(&self.path_along_edge(node_a, inchain_a.end_a) 
            + &self.find_path(inchain_a.end_a, inchain_a.end_b))
            + &self.path_along_edge(inchain_a.end_b, node_b);
          let path3 = &(&self.path_along_edge(node_a, inchain_a.end_b) 
            + &self.find_path(inchain_a.end_b, inchain_a.end_a))
            + &self.path_along_edge(inchain_a.end_a, node_b);
          let min_path = if path1 < path2 { path1 } else { path2 };
          if min_path < path3 { min_path } else { path3 }
        } else {
          let path1 = &self.path_along_edge(node_a, inchain_a.end_a) + 
            &self.find_path(inchain_a.end_a, node_b);
          let path2 = &self.path_along_edge(node_a, inchain_a.end_b) + 
            &self.find_path(inchain_a.end_b, node_b);
          if path1 < path2 {
            path1
          } else {
            path2
          }
        }
      },
    }
  }
}
