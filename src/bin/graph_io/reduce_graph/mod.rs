use std::rc::Rc;
use std::cmp::Ordering::*;
use crate::parse_edge_txt::{edge_file_to_vec, store_edges, store_vec};
use crate::types::*;

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

fn sort_edges_by_order(edges : &mut Vec<Rc<Edge>>) {
  edges.sort_by(|a,b| (*a.order.borrow()).cmp(&(*b.order.borrow())));
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

fn sort_edges_by_nodes(edges : &mut Vec<Rc<Edge>>) {
  edges.sort_by(|a,b| a.node_a.cmp(&b.node_a).then_with(|| a.node_b.cmp(&b.node_b)));
}

fn updated_reduced_edges(edges : &Vec<Rc<Edge>>, nodes : &Vec<Node>) -> Vec<Rc<Edge>> {
  let mut reduced_edges = Vec::with_capacity(edges.len());
  let mut edge_iter = edges.iter();
  let mut curr_edge : &Rc<Edge>;

  match edge_iter.next() {
    None => return reduced_edges,
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
            reduced_edges.push(Rc::new(Edge::new(*i, *j, curr_edge.distance)));
          },
          _ => (),
        };
        curr_edge = next_edge;
      },
      Some(Greater) => curr_edge = next_edge,
      _ => (),
    }
  }
  match (&nodes[curr_edge.node_a].reduced_state, &nodes[curr_edge.node_b].reduced_state) {
    (ReducedState::INGRAPH(i), ReducedState::INGRAPH(j)) => {
      reduced_edges.push(Rc::new(Edge::new(*i, *j, curr_edge.distance)));
    },
    _ => (),
  };

  return reduced_edges;
}

fn remove_val_2_nodes (input_file : &str) -> (Vec<Rc<Edge>>, Vec<Chain>) {
  let (mut edges, num_nodes) = edge_file_to_vec(input_file);
  let nodes = create_node_vec(&edges, num_nodes);
  
  visit_node(&nodes, 0, 0);
  sort_edges_by_order(&mut edges);

  remove_edges(edges, &nodes)
}


pub fn complete_reduction(input_file : &str, output_edges : &str, output_nodes : &str, output_chains : &str) {
  let (mut edges, num_nodes) = edge_file_to_vec(input_file);
  let mut nodes = create_node_vec(&edges, num_nodes);
  
  visit_node(&nodes, 0, 0);
  sort_edges_by_order(&mut edges);

  let (mut reduced_edges, chains) = remove_edges(edges, &nodes);
  update_chain_nodes(&chains, &mut nodes);
  update_remain_nodes(&mut nodes);

  let reduced_nodes = Node::get_reduced(&nodes);
  store_vec(&reduced_nodes, output_nodes);
  store_vec(&chains, output_chains);

  sort_edges_by_nodes(&mut reduced_edges);
  //let deduplicated = remove_duplicate_edges(reduced_edges);
  let updated = updated_reduced_edges(&reduced_edges, &nodes);

  store_edges(updated, output_edges)
}

#[cfg(test)]
mod tests;
