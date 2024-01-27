use std::cell::RefCell;
use std::fs::{File, self};
use std::io::{self, BufRead};
use std::rc::Rc;
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

fn identify_val_2(edges : &Vec<Rc<Edge>>, num_nodes : usize) -> Vec<NodeState>{
  let mut nodes_status = Vec::with_capacity(num_nodes);
  for _ in 0..num_nodes {
    nodes_status.push(NodeState::new());
  }
  for edge in edges { 
    nodes_status[(*edge).node_a].add_link();
    nodes_status[(*edge).node_b].add_link();
  }
  nodes_status
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
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.node_a == other.node_a 
          && self.node_b == other.node_b 
          && self.distance == other.distance

    }
}

struct Node {
    rc_edge : Vec<Rc<Edge>>,
    visited : RefCell<bool>,
}

impl Node {
    fn new() -> Self {
        Node { rc_edge : Vec::new(), visited : RefCell::new(false)}
    }

    fn add_edge(&mut self, edge : &Rc<Edge>) {
      self.rc_edge.push(Rc::clone(edge));
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
  for _ in 0..num_nodes {
    nodes.push(Node::new());
  }
  for edge in edges { 
    nodes[(*edge).node_a].add_edge(&edge);
    nodes[(*edge).node_b].add_edge(&edge);
  }
  nodes
}

fn sort_edges(edges : &mut Vec<Rc<Edge>>) {
  edges.sort_by(|a,b| (*a.order.borrow()).cmp(&(*b.order.borrow())));
}

fn check_connected(a : &Rc<Edge>, b : &Rc<Edge>) -> Option<(bool,bool)>{ 
  match a.node_a {
    n if n == b.node_a => Some((true, true)),
    n if n == b.node_b => Some((true, false)),
    _ => match a.node_b {
      n if n == b.node_a => Some((false, true)),
      n if n == b.node_b => Some((false, false)),
      _ => None,
    },
  }
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
}

fn add_edge(chain : &Chain, edges : &mut Vec<Rc<Edge>>) {
  let mut ends = [chain.start, chain.end];
  ends.sort();
  edges.push(Rc::new(Edge::new(ends[0], ends[1], chain.dist)));
}

fn remove_edges(edges : Vec<Rc<Edge>>, nodes_state : &Vec<NodeState>) -> (Vec<Rc<Edge>>, Vec<Chain>) {
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
    match check_connected(&curr_edge, &next_edge) {
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
          add_edge(&chain, &mut new_edges);
          chains.push(chain);
          chain = Chain::new();
          in_chain = false;
        }
      },
      Some((curr_common, curr_next)) => {
        let shared : usize;
        if curr_common {
          shared = curr_edge.node_a;
        } else {
          shared = curr_edge.node_b;
        }
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
            add_edge(&chain, &mut new_edges);
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
    add_edge(&chain, &mut new_edges);
    chains.push(chain);
  }

    
  (new_edges, chains)
}

fn sort_val_2 (input_file : File) -> Vec<Rc<Edge>> {
  let (mut edges, num_nodes) = edge_file_to_vec(input_file);
  let nodes = create_node_vec(&edges, num_nodes);
  
  visit_node(&nodes, 0, 0);
  sort_edges(&mut edges);


  edges
}

fn remove_val_2_nodes (input_file : File) -> (Vec<Rc<Edge>>, Vec<Chain>) {
  let (mut edges, num_nodes) = edge_file_to_vec(input_file);
  let nodes = create_node_vec(&edges, num_nodes);
  
  visit_node(&nodes, 0, 0);
  sort_edges(&mut edges);
  
  let nodes_state = identify_val_2(&edges, num_nodes);

  remove_edges(edges, &nodes_state)
}

#[cfg(test)]
mod tests;
