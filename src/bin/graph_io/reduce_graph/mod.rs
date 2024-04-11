use std::cell::RefCell;
use std::fs::File;
use serde::{Serialize,Deserialize};
use std::io::{self, Write, BufRead};
use std::rc::Rc;
use std::cmp::Ordering::*;
use crate::parse_edge_txt::parse_string;

#[derive(Serialize, Deserialize)]
struct ChainState {
  chain_id : usize,
  end_a : usize,
  dist_a : f64,
  end_b : usize,
  dist_b : f64,
}

impl ChainState {
  fn new (chain_id : usize, end_a : usize, dist_a : f64, end_b : usize, dist_b : f64)  -> ChainState {
    ChainState { chain_id , end_a , dist_a , end_b, dist_b }
  }
}


#[derive(Serialize, Deserialize)]
enum ReducedState {
  INCHAIN(ChainState),
  INGRAPH(usize)
}

struct Node{
  rc_edge : Vec<Rc<Edge>>,
  visited : RefCell<bool>,
  reduced_state : ReducedState
}

impl Node {
  fn new (index : usize)  -> Node {
    Node { rc_edge : Vec::new(), visited : RefCell::new(false), reduced_state : ReducedState::INGRAPH(index) }
  }

  fn is_val_2 (&self) -> bool {
   self.rc_edge.len() == 2 
  }

  fn add_edge(&mut self, edge : &Rc<Edge>) {
    self.rc_edge.push(Rc::clone(edge));
  }
}

#[derive(Debug)]
struct Edge {
    node_a : usize,
    node_b : usize,
    distance : f64,
    visited: RefCell<bool>,
    order: RefCell<usize>,
}

impl Edge {
    fn new(node_a : usize, node_b : usize, distance : f64) -> Self {
        Edge { node_a,
          node_b,
          distance,
          visited : RefCell::new(false),
          order :  RefCell::new(0)
        }
    }

  fn is_connected(&self, b : &Edge) -> Option<(bool,bool)>{ 
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

fn visit_node (nodes : &Vec<Node>, node_index : usize, mut order : usize) -> usize {
  let mut next_node : usize; 
  {
    let mut rf_visited = nodes[node_index].visited.borrow_mut();
    if *rf_visited {
      return order;
    }
    *rf_visited = true;
  }
  for edge in nodes[node_index].rc_edge.iter() {
    { 
      let mut rf_visited = (*edge).visited.borrow_mut();
      if *rf_visited == true {
        continue;
      }
      let mut rf_order = (*edge).order.borrow_mut();
      *rf_visited = true;
      *rf_order = order;
      if node_index == (*edge).node_a {
        next_node = (*edge).node_b;
      } else {
        next_node = (*edge).node_a;
      }
    }
    order += 1;
    order = visit_node(nodes, next_node, order);
  }
  order
}

fn edge_file_to_vec(input_file : File) -> (Vec<Rc<Edge>>, usize) {
  let input_reader = io::BufReader::new(input_file);
  let mut edges : Vec<Rc<Edge>> = Vec::new();
  let mut num_nodes = 0;
  // Iterate over the lines in the file
  for line in input_reader.lines() {
      // Handle each line
      match line {
          Ok(content) => {
            match parse_string(&content) {
              Ok((start_node, end_node, distance)) =>  {
                if end_node >= num_nodes {
                  num_nodes = end_node + 1;
                }
                edges.push(Rc::new(Edge::new(start_node, end_node, distance)))
              }
              ,
              Err(err) => eprintln!("Error reading line: {}", err),
            }
          }
          Err(err) => eprintln!("Error reading line: {}", err),
      }
  }
  (edges, num_nodes)
}

fn create_node_vec(edges : &Vec<Rc<Edge>>, num_nodes : usize) -> Vec<Node> {
  let mut nodes : Vec<Node> = Vec::with_capacity(num_nodes);
  for i in 0..num_nodes {
    nodes.push(Node::new(i));
  }
  for edge in edges { 
    nodes[(*edge).node_a].add_edge(&edge);
    nodes[(*edge).node_b].add_edge(&edge);
  }
  nodes
}

fn sort_edges_by_order(edges : &mut Vec<Rc<Edge>>) {
  edges.sort_by(|a,b| (*a.order.borrow()).cmp(&(*b.order.borrow())));
}

#[derive(Debug,PartialEq)]
struct Chain {
  start : usize,
  end : usize, 
  nodes : Vec<(usize, f64)>,
  dist : f64,
}

impl Chain {
  fn new() -> Self {
    Chain { start : 0, end : 0, nodes : Vec::new(), dist : 0.0 }
  }

  fn get_edge(&self) -> Rc<Edge>{
    let mut ends = [self.start, self.end];
    ends.sort();
    Rc::new(Edge::new(ends[0], ends[1], self.dist))
  }
}


fn remove_edges(edges : Vec<Rc<Edge>>, nodes_state : &Vec<Node>) -> (Vec<Rc<Edge>>, Vec<Chain>) {
  let mut edge_iter = edges.into_iter();
  let mut new_edges = Vec::new();
  let mut chains = Vec::new();
  let mut curr_edge : Rc<Edge>;
  match edge_iter.next() {
    None => return (new_edges, chains),
    Some(edge) => curr_edge = edge,
  }
  
  let mut in_chain = false;
  let mut previous = false;
  let mut chain = Chain::new();

  for next_edge in edge_iter {
    match curr_edge.is_connected(&next_edge) {
      None => {
        if !in_chain {
          new_edges.push(curr_edge);
        } else {
          chain.dist += curr_edge.distance;
          match previous {
            true => chain.end = curr_edge.node_b,
            false => chain.end = curr_edge.node_a,
          }
          chain.nodes.push((chain.end, chain.dist));
          new_edges.push(chain.get_edge());
          chains.push(chain);
          chain = Chain::new();
          in_chain = false;
        }
      },
      Some((curr_common, curr_next)) => {
        let shared = match curr_common {
          true => curr_edge.node_a,
          false => curr_edge.node_b,
        };
        if nodes_state[shared].is_val_2() {
          if !in_chain {
            in_chain = true;
            match curr_common {
              true => chain.start = curr_edge.node_b,
              false => chain.start = curr_edge.node_a,
            }
            chain.nodes.push((chain.start, chain.dist));
          }
          chain.dist += curr_edge.distance;
          chain.nodes.push((shared, chain.dist));
          previous = curr_next;
        } else {
          if !in_chain {
            new_edges.push(curr_edge);
          }
          else {
            chain.dist += curr_edge.distance;
            chain.end = shared;
            chain.nodes.push((chain.end, chain.dist));
            new_edges.push(chain.get_edge());
            chains.push(chain);
            chain = Chain::new();
            in_chain = false;
          }           
        }
      }
    }
    curr_edge = next_edge;
  }
  if !in_chain {
    new_edges.push(curr_edge);
  } else {
    chain.dist += curr_edge.distance;
    match previous {
      true => chain.end = curr_edge.node_b,
      false => chain.end = curr_edge.node_a,
    }
    chain.nodes.push((chain.end, chain.dist));
    new_edges.push(chain.get_edge());
    chains.push(chain);
  }
    
  (new_edges, chains)
}

fn update_chain_nodes (chains : &Vec<Chain>, nodes : &mut Vec<Node>) {
  for (chain_id, chain) in chains.iter().enumerate() {
    let len = chain.nodes.len() - 2;
    for &(node_id, dist) in chain.nodes.iter().skip(1).take(len) {
      nodes[node_id].reduced_state = ReducedState::INCHAIN( 
        ChainState::new(chain_id, chain.start, dist, chain.end, chain.dist - dist)
        )
    }
  }
}

fn update_remain_nodes (nodes : &mut Vec<Node>) {
  let mut reduced_index = 0;
  for node in nodes {
    match node.reduced_state {
      ReducedState::INGRAPH(_) => {
        node.reduced_state = ReducedState::INGRAPH(reduced_index);
        reduced_index += 1;
      }
      _ => continue
    }
  }

}

fn remove_val_2_nodes (input_file : File) -> (Vec<Rc<Edge>>, Vec<Chain>) {
  let (mut edges, num_nodes) = edge_file_to_vec(input_file);
  let nodes = create_node_vec(&edges, num_nodes);
  
  visit_node(&nodes, 0, 0);
  sort_edges_by_order(&mut edges);

  remove_edges(edges, &nodes)
}

fn chain_to_io(chain : Chain, id : usize) -> Vec<String> {
  let order = chain.start < chain.end;
  let len = chain.nodes.len() - 2;

  let node_to_io = |node : usize, dist : f64| {
    let str : String;
    if order {
      str = format!("{} {} {} {} {} {}",
                        node, id,chain.start, dist, chain.end, chain.dist - dist);
    } else {
      str = format!("{} {} {} {} {} {}",
                        node, id, chain.end, chain.dist - dist, chain.start, dist);
    }
    str
  };

  chain.nodes.iter().skip(1).take(len).map(|&(node, dist)| node_to_io(node,dist)).collect::<Vec<_>>()
}

fn store_val_2(chains : Vec<Chain>, output_file_path : &str) -> io::Result<()> {
  let formatted_strings : Vec<String> = chains.into_iter().enumerate()
    .flat_map(|(id, chain)| chain_to_io(chain, id))
    .collect();

  let mut file = File::create(output_file_path)?;
  for line in formatted_strings{
    writeln!(file, "{}", line)?;
  }
  Ok(())
}


fn sort_edges_by_nodes(edges : &mut Vec<Rc<Edge>>) {
  edges.sort_by(|a,b| a.node_a.cmp(&b.node_a).then_with(|| a.node_b.cmp(&b.node_b)));
}

fn remove_duplicate_edges(edges : Vec<Rc<Edge>>) -> Vec<Rc<Edge>> {
  let mut deduplicated_edges = Vec::with_capacity(edges.len());
  let mut edge_iter = edges.into_iter();
  let mut curr_edge : Rc<Edge>;

  match edge_iter.next() {
    None => return deduplicated_edges,
    Some(edge) => curr_edge = edge,
  }

  for next_edge in edge_iter {
    if (*curr_edge).node_a == (*curr_edge).node_b {
      curr_edge = next_edge;
      continue;
    }
    match (*curr_edge).partial_cmp(&(*next_edge)) {
      None => {
        deduplicated_edges.push(curr_edge);
        curr_edge = next_edge;
      },
      Some(Greater) => curr_edge = next_edge,
      _ => (),
    }
  }
  deduplicated_edges.push(curr_edge);

  return deduplicated_edges;
}

fn updated_reduced_edges(edges : Vec<Rc<Edge>>, nodes : Vec<Node>) -> Vec<Rc<Edge>> {
  let mut deduplicated_edges = Vec::with_capacity(edges.len());
  let mut edge_iter = edges.into_iter();
  let mut curr_edge : Rc<Edge>;

  match edge_iter.next() {
    None => return deduplicated_edges,
    Some(edge) => curr_edge = edge,
  }

  for next_edge in edge_iter {
    if (*curr_edge).node_a == (*curr_edge).node_b {
      curr_edge = next_edge;
      continue;
    }
    match (*curr_edge).partial_cmp(&(*next_edge)) {
      None => {
        match (&nodes[curr_edge.node_a].reduced_state, &nodes[curr_edge.node_b].reduced_state) {
          (ReducedState::INGRAPH(i), ReducedState::INGRAPH(j)) => {
            deduplicated_edges.push(Rc::new(Edge::new(*i, *j, curr_edge.distance)));
          },
          _ => (),
        };
        deduplicated_edges.push(curr_edge);
        curr_edge = next_edge;
      },
      Some(Greater) => curr_edge = next_edge,
      _ => (),
    }
  }
  deduplicated_edges.push(curr_edge);

  return deduplicated_edges;
}
fn store_edges(edges : Vec<Rc<Edge>>, output_file_path : &str) -> io::Result<()> {
  let formatted_strings : Vec<String> = edges.into_iter().enumerate()
    .map(|(id, edge)| format!("{} {} {} {}", id, edge.node_a, edge.node_b, edge.distance))
    .collect();

  let mut file = File::create(output_file_path)?;
  for line in formatted_strings{
    writeln!(file, "{}", line)?;
  }
  Ok(())
}

fn store_nodes(nodes : &Vec<Node>, output_file_path : &str) -> io::Result<()> {
  let mut file = File::create(output_file_path)?;
  for node in nodes {
    let serialized = bincode::serialize(&node.reduced_state).unwrap();
    file.write_all(&serialized)?;
  }
  Ok(())
}

pub fn complete_reduction(input_file : &str, output_edges : &str, output_nodes : &str) 
  -> io::Result<()> {
  let input_file = File::open(input_file)?;
  let (mut edges, num_nodes) = edge_file_to_vec(input_file);
  let mut nodes = create_node_vec(&edges, num_nodes);
  
  visit_node(&nodes, 0, 0);
  sort_edges_by_order(&mut edges);

  let (mut reduced_edges, chains) = remove_edges(edges, &nodes);
  update_chain_nodes(&chains, &mut nodes);
  store_val_2(chains, output_nodes)?;

  sort_edges_by_nodes(&mut reduced_edges);
  //let deduplicated = remove_duplicate_edges(reduced_edges);
  let updated = updated_reduced_edges(reduced_edges, nodes);

  store_edges(updated, output_edges)
}

#[cfg(test)]
mod tests;
