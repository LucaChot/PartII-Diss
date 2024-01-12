use crate::matrix_multiplication::Matrix;
use super::Val2Node;

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

fn get_col<T : Copy>(matrix : &Matrix<T>, col : usize) -> Vec<T>{
  matrix.iter()
    .map(|links_out| links_out[col])
    .collect::<Vec<_>>()
}

fn update_matrix_values(matrix_w : &mut Matrix<isize>, matrix_p : &mut Matrix<usize>) -> Vec<Val2Node> {
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

pub fn get_remaining_nodes(removed_nodes : &Vec<Val2Node>, num_nodes : usize) -> Vec<usize> {
  let mut remaining_nodes : Vec<bool> = vec![true; num_nodes];
  let _ = removed_nodes.iter().map(|val_2_node| remaining_nodes[val_2_node.node] = false).collect::<Vec<_>>();
  remaining_nodes.iter().enumerate()
    .filter(|&(_, &entry)| entry).map(|(index, _)| index).collect()
}

fn get_reduced_matrix_slice<T:Copy>(matrix : &Matrix<T>, remaining_nodes : &Vec<usize>) -> Matrix<T>{
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

pub fn remove_val2_nodes (matrix_w : &Matrix<isize>,matrix_p : &Matrix<usize>) -> 
  (Matrix<isize>, Matrix<usize>, Vec<Val2Node>) {
  let mut matrix_w_clone = matrix_w.clone();
  let mut matrix_p_clone = matrix_p.clone();

  let num_nodes = matrix_p.len();
  let removed_nodes = update_matrix_values(&mut matrix_w_clone, &mut matrix_p_clone);
  let remaining_nodes = get_remaining_nodes(&removed_nodes, num_nodes);

  let reduced_w = get_reduced_matrix_slice(&matrix_w_clone, &remaining_nodes);
  let reduced_p = get_reduced_matrix_slice(&matrix_p_clone, &remaining_nodes);

  (reduced_w, reduced_p, removed_nodes)
}

#[cfg(test)]
mod tests;
