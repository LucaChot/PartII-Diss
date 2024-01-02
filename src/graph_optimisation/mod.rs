use crate::matrix_multiplication::{Matrix};

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
pub fn remove_val2_nodes (mut matrix_w : Matrix<isize>, mut matrix_p : Matrix<usize>) -> (Matrix<isize>, Matrix<usize>) {
  let num_nodes = matrix_w.len();
  // Tracks which nodes will remain after the pruning
  let mut remaining_nodes : Vec<bool> = vec![true; num_nodes];
  let mut map_to_reduced_index : Vec<usize> = vec![0; num_nodes];
  let mut current_reduced_node_count = 0;
  // Iterate over each node and test whether it has valency-2
  for node in 0..num_nodes {
    let links_in : Vec<usize> = matrix_p.iter()
      .map(|links_out| links_out[node])
      .collect::<Vec<_>>();
    let links_out = &matrix_p[node];

    match is_val2_nodes(node, &links_in, links_out) {
      None => {
        map_to_reduced_index[node] = current_reduced_node_count;
        current_reduced_node_count += 1;
      }
      ,
      // Function returns an Option<i32> where the value returned is the index
      // of the next node
      Some((node_from, node_to)) => {
        // Checks that next node and previous node aren't connected to ensure
        // that the valency of other nodes are kept the same
        if matrix_p[node_from][node_to] == node_from {
          map_to_reduced_index[node] = current_reduced_node_count;
          current_reduced_node_count += 1;
          continue;
        }

        remaining_nodes[node] = false;
        // New distance to next node and updates the new weight
        let new_distance = matrix_w[node_from][node] + matrix_w[node][node_to];
        matrix_w[node_from][node_to] = new_distance;
        matrix_p[node_from][node_to] = node_from;
        // Removes the old node entry
        matrix_p[node_from][node] = node;
        matrix_w[node_from][node] = -1;
        matrix_p[node][node_to] = node_to;
        matrix_w[node][node_to] = -1;
      },
    };
  }

  let _ = matrix_p.iter_mut()
    .map(|row| row.iter_mut()
      .map(|node| *node = map_to_reduced_index[*node])
      .collect::<Vec<_>>())
    .collect::<Vec<_>>();


  dbg!(&remaining_nodes);

  let mut reduced_w : Matrix<isize> = Vec::new();
  let mut reduced_p : Matrix<usize> = Vec::new();
  let remaining_nodes_iter = remaining_nodes.iter().enumerate()
    .filter(|&(_, &entry)| entry).map(|(index, _)| index);
  
  for i in remaining_nodes_iter.clone() {
    let mut row : Vec<isize> = Vec::new();
    for j in remaining_nodes_iter.clone() {
      row.push(matrix_w[i][j]);
    }
    reduced_w.push(row);
  }

  for i in remaining_nodes_iter.clone() {
    let mut row : Vec<usize> = Vec::new();
    for j in remaining_nodes_iter.clone() {
      row.push(matrix_p[i][j]);
    }
    reduced_p.push(row);
  }

  return (reduced_w, reduced_p);
}

#[cfg(test)]
mod tests;
