use crate::matrix_multiplication::{Matrix};

pub struct Val2Node {
  node : usize,
  node_from : usize,
  node_from_dist : isize,
  node_to : usize,
  node_to_dist : isize,
}

impl Val2Node {
  pub fn new(node : usize, node_from : usize, node_from_dist : isize, node_to : usize, node_to_dist : isize) -> Val2Node {
    Val2Node { node, node_from, node_from_dist, node_to, node_to_dist }
  }
}

// TODO: Potentially improve performance be reducing number of is_val2_nodes calls
// and implementing a counter
// TODO: Devise a means of recomputing the path from an optimised graph

fn is_val2_nodes(node:usize, links_in : &Vec<usize>, links_out : &Vec<usize>) -> Option<(usize,usize)> {
  // Gets all the nodes that have a link from node
  let num_links_in : Vec<usize> = links_in.iter().filter(|&&node_from| node_from != node)
    .map(|&node_from| node_from).collect();
  // Gets all the nodes that have a link from node
  let num_links_out : Vec<usize> = links_out.iter().enumerate().filter(|(node_to, &x)| x == node && *node_to != node)
    .map(|(node_to, _)| node_to).collect();
  // Checks whether both the number of nodes in and out are 1 and returns the node_from and the
  // node_to
  if num_links_in.len() == 1 && num_links_out.len() == 1 {
    return Some((num_links_in[0], num_links_out[0]));
  }
  return None;
}

pub fn get_col<T : Copy>(matrix : &Matrix<T>, col : usize) -> Vec<T>{
  matrix.iter()
    .map(|links_out| links_out[col])
    .collect::<Vec<_>>()
}

pub fn update_matrix_values(matrix_w : &mut Matrix<isize>, matrix_p : &mut Matrix<usize>) -> Vec<Val2Node> {
  let num_nodes = matrix_w.len();
  let mut removed_nodes : Vec<Val2Node> = Vec::with_capacity(num_nodes);

  // Iterate over each node and test whether it has valency-2
  for node in 0..num_nodes {
    let links_in : Vec<usize> = get_col(matrix_p, node);
    let links_out = &matrix_p[node];

    match is_val2_nodes(node, &links_in, links_out) {
      None => {
      }
      ,
      // Function returns an Option<i32> where the value returned is the index
      // of the next node
      Some((node_from, node_to)) => {
        // Checks that next node and previous node aren't connected to ensure
        // that the valency of other nodes are kept the same
        if matrix_p[node_from][node_to] == node_from {
          continue;
        }
        let node_from_dist = matrix_w[node_from][node];
        let node_to_dist = matrix_w[node][node_to];
        removed_nodes.push(Val2Node{node,node_from,node_from_dist,node_to,node_to_dist});
        // New distance to next node and updates the new weight
        let new_distance = node_from_dist + node_to_dist;
        matrix_w[node_from][node_to] = new_distance;
        matrix_p[node_from][node_to] = node_from;
        // Removes the old node entry
        matrix_p[node_from][node] = node;
        matrix_w[node_from][node] = -1;
        matrix_p[node][node_to] = node_to;
        matrix_w[node][node_to] = -1;
      },
    }
  }
  
  removed_nodes
}

pub fn get_removed_nodes_iter(removed_nodes : &Vec<Val2Node>, num_nodes : usize) -> Vec<usize> {
  let mut remaining_nodes : Vec<bool> = vec![true; num_nodes];
  let _ = removed_nodes.iter().map(|val_2_node| remaining_nodes[val_2_node.node] = false).collect::<Vec<_>>();
  remaining_nodes.iter().enumerate()
    .filter(|&(_, &entry)| entry).map(|(index, _)| index).collect()
}

pub fn get_reduced_matrix_slice<T:Copy>(matrix : &Matrix<T>, remaining_nodes : &Vec<usize>) -> Matrix<T>{
  let mut reduced : Matrix<T> = Vec::new();

  for &i in remaining_nodes.iter() {
    let mut row : Vec<T> = Vec::new();
    for &j in remaining_nodes.iter() {
      row.push(matrix[i][j]);
    }
    reduced.push(row);
  }
  reduced
}

/// Algorithm Idea:
/// 1. Find number of valency-2 nodes and add to a queue (make note of nodes 
/// being in the queue)
/// - May not need to make note of nodes being in queue as if we needed to add
/// node to queue later it musn't have been one already as we add nodes that 
/// where connected to current node and destination node
/// 2. Iterate through queue and refcord distance to next node and its index 
/// and connected nodes
/// 3. Remove it and later nodes before it
/// 4. If the bode before is connected to node after choose shortest path and
/// add to queue if it becomes a valency-2 node
/// 5. Repeat till queue is empty
pub fn new_remove_val2_nodes (matrix_w : &Matrix<isize>,matrix_p : &Matrix<usize>) -> 
  (Matrix<isize>, Matrix<usize>, Vec<Val2Node>) {
  let mut matrix_w_clone = matrix_w.clone();
  let mut matrix_p_clone = matrix_p.clone();

  let num_nodes = matrix_p.len();
  let removed_nodes = update_matrix_values(&mut matrix_w_clone, &mut matrix_p_clone);
  let remaining_nodes = get_removed_nodes_iter(&removed_nodes, num_nodes);

  let reduced_w = get_reduced_matrix_slice(&matrix_w_clone, &remaining_nodes);
  let reduced_p = get_reduced_matrix_slice(&matrix_p_clone, &remaining_nodes);

  (reduced_w, reduced_p, removed_nodes)
}

fn add_removed_col_p(matrix : &mut Matrix<usize>, val_2_node : &Val2Node) {
  let node_from_col_iter = matrix.iter().map(|row| row[val_2_node.node_from]).collect::<Vec<_>>();
  let node_col_mut_iter = matrix.iter_mut().map(|row| &mut row[val_2_node.node]);

  node_col_mut_iter.zip(node_from_col_iter).enumerate().map(
    |(index, (node_col, node_from_col))| {
      if node_from_col != val_2_node.node_from || index == val_2_node.node_from {
        *node_col = val_2_node.node_from;
      } else {
        *node_col = val_2_node.node;
      }
    }
  ).count();
}

fn add_removed_row_p(matrix : &mut Matrix<usize>, val_2_node : &Val2Node) {
  let node_to_row = matrix[val_2_node.node_to].clone();
  matrix[val_2_node.node].iter_mut().zip(node_to_row).enumerate().map(
    |(index, (node_row_item, node_to_row_item))| {
      if index == val_2_node.node || index == val_2_node.node_to {
        *node_row_item = val_2_node.node;
      } else {
        *node_row_item = node_to_row_item;
      }
    }
  ).count();
}

fn update_removed_node_to_p(matrix : &mut Matrix<usize>, val_2_node : &Val2Node) {
  let node_to_col_mut_iter = matrix.iter_mut().map(|row| &mut row[val_2_node.node_to]);
  let _ = node_to_col_mut_iter.map(|node_to_col_item| {
    if *node_to_col_item == val_2_node.node_from {
      *node_to_col_item = val_2_node.node;
    }
  }).count();
}


pub fn recover_val2_nodes_p (reduced_p : &Matrix<usize>, matrix_p : &Matrix<usize>, 
                             rm_nodes : &Vec<Val2Node>) -> Matrix<usize> {
  let num_rows = matrix_p.len();
  let num_cols = matrix_p[0].len();
  //let mut removed_nodes = (*rm_nodes).clone();
  let remaining_nodes = get_removed_nodes_iter(rm_nodes, num_rows);

  let mut reconstructed_p : Matrix<usize> = vec![(0..num_rows).collect(); num_rows];
  let reduced_p_indices_iter = remaining_nodes.iter();   

  let zip_reduced_p_indices_iter = reduced_p.iter().zip(reduced_p_indices_iter.clone());

  let _ = zip_reduced_p_indices_iter.map(|(rows, &node)|
    rows.iter().zip(reduced_p_indices_iter.clone()).map(|(&pred, &target)|
      reconstructed_p[node][target] = pred).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

  rm_nodes.iter().rev().map(|val_2_node| {
    add_removed_col_p(&mut reconstructed_p, val_2_node);
    add_removed_row_p(&mut reconstructed_p, val_2_node);
    update_removed_node_to_p(&mut reconstructed_p, val_2_node);
  }).count();
    
  reconstructed_p
}

fn add_removed_col_w(matrix : &mut Matrix<isize>, val_2_node : &Val2Node) {
  let node_from_col_iter = matrix.iter().map(|row| row[val_2_node.node_from]).collect::<Vec<_>>();
  let node_col_mut_iter = matrix.iter_mut().map(|row| &mut row[val_2_node.node]);

  node_col_mut_iter.zip(node_from_col_iter).enumerate().map(
    |(index, (node_col_item, node_from_col_item))| {
      if index == val_2_node.node {
        *node_col_item = 0
      } else if node_from_col_item != -1 {
        *node_col_item = node_from_col_item + val_2_node.node_from_dist;
      }
    }
  ).count();
}

fn add_removed_row_w(matrix : &mut Matrix<isize>, val_2_node : &Val2Node) {
  let node_to_row = matrix[val_2_node.node_to].clone();
  matrix[val_2_node.node].iter_mut().zip(node_to_row).enumerate().map(
    |(index, (node_row_item, node_to_row_item))| {
      if node_to_row_item != -1 && index != val_2_node.node {
        *node_row_item = node_to_row_item + val_2_node.node_to_dist;
      }
    }
  ).count();
}

pub fn recover_val2_nodes_w (reduced_p : &Matrix<isize>, matrix_p : &Matrix<isize>, 
                             rm_nodes : &Vec<Val2Node>) -> Matrix<isize> {
  let num_rows = matrix_p.len();
  let num_cols = matrix_p[0].len();
  let remaining_nodes = get_removed_nodes_iter(rm_nodes, num_rows);

  let mut reconstructed_p : Matrix<isize> = vec![(0..num_cols).map(|_| -1).collect(); num_rows];
  let reduced_p_indices_iter = remaining_nodes.iter();   

  let zip_reduced_p_indices_iter = reduced_p.iter().zip(reduced_p_indices_iter.clone());

  let _ = zip_reduced_p_indices_iter.map(|(rows, &node)|
    rows.iter().zip(reduced_p_indices_iter.clone()).map(|(&pred, &target)|
      reconstructed_p[node][target] = pred).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

  rm_nodes.iter().rev().map(|val_2_node| {
    add_removed_col_w(&mut reconstructed_p, val_2_node);
    add_removed_row_w(&mut reconstructed_p, val_2_node);
  }).count();
    
  reconstructed_p
}

#[cfg(test)]
mod tests;
