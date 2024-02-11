use crate::types::Matrix;
use super::Val2Node;

use super::reduction::get_remaining_nodes;

fn update_p_col_item(val_2_node : &Val2Node, index : usize, node_col_item : &mut usize, node_from_col_item : usize) {
  if node_from_col_item != val_2_node.node_from || index == val_2_node.node_from {
    *node_col_item = val_2_node.node_from;
  } else {
    *node_col_item = val_2_node.node;
  }
}

fn update_w_col_item(val_2_node : &Val2Node, index : usize, node_col_item : &mut isize, node_from_col_item : isize) {
  if index == val_2_node.node {
    *node_col_item = 0
  } else if node_from_col_item != -1 {
    *node_col_item = node_from_col_item + val_2_node.node_from_dist;
  }
}

fn add_removed_col<T : Copy>(matrix : &mut Matrix<T>, val_2_node : &Val2Node, update_col_item : fn(&Val2Node, usize, &mut T, T)) {
  let node_from_col = matrix.iter().map(|row| row[val_2_node.node_from]).collect::<Vec<_>>();
  let node_col_mut_iter = matrix.iter_mut().map(|row| &mut row[val_2_node.node]);

  node_col_mut_iter.zip(node_from_col).enumerate().map(
    |(index, (node_col_item, node_from_col_item))| update_col_item(val_2_node, index, node_col_item, node_from_col_item)
  ).count();
}

fn update_p_row_item(val_2_node : &Val2Node, index : usize, node_row_item : &mut usize, node_to_row_item : usize) {
  if index == val_2_node.node || index == val_2_node.node_to {
    *node_row_item = val_2_node.node;
  } else {
    *node_row_item = node_to_row_item;
  }
}

fn update_w_row_item(val_2_node : &Val2Node, index : usize, node_row_item : &mut isize, node_to_row_item : isize) {
    if node_to_row_item != -1 && index != val_2_node.node {
      *node_row_item = node_to_row_item + val_2_node.node_to_dist;
    }
}

fn add_removed_row<T : Copy>(matrix : &mut Matrix<T>, val_2_node : &Val2Node, update_row_item : fn(&Val2Node, usize, &mut T, T)) {
  let node_to_row = matrix[val_2_node.node_to].clone();
  matrix[val_2_node.node].iter_mut().zip(node_to_row).enumerate().map(
    |(index, (node_row_item, node_to_row_item))| update_row_item(val_2_node, index, node_row_item, node_to_row_item)
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

fn add_removed_node_p(reconstructed_p : &mut Matrix<usize>, val_2_node : &Val2Node) {
    add_removed_col(reconstructed_p, val_2_node, update_p_col_item);
    add_removed_row(reconstructed_p, val_2_node, update_p_row_item);
    update_removed_node_to_p(reconstructed_p, val_2_node);
}

fn add_removed_node_w(reconstructed_w : &mut Matrix<isize>, val_2_node : &Val2Node) {
    add_removed_col(reconstructed_w, val_2_node, update_w_col_item);
    add_removed_row(reconstructed_w, val_2_node, update_w_row_item);
}

fn recover_val2_nodes<T : Copy>(reduced : &Matrix<T>, matrix : &Matrix<T>, start : &Matrix<T>,
                             rm_nodes : &Vec<Val2Node>, add_removed_node : fn(&mut Matrix<T>,&Val2Node))-> Matrix<T> {
  let num_rows = matrix.len();
  let remaining_nodes = get_remaining_nodes(rm_nodes, num_rows);

  let mut reconstructed : Matrix<T> = start.clone();
  let reduced_indices_iter = remaining_nodes.iter();   

  let zip_reduced_indices_iter = reduced.iter().zip(reduced_indices_iter.clone());

  let _ = zip_reduced_indices_iter.map(|(rows, &node)|
    rows.iter().zip(reduced_indices_iter.clone()).map(|(&pred, &target)|
      reconstructed[node][target] = pred).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

  rm_nodes.iter().rev().map(|val_2_node| {
    add_removed_node(&mut reconstructed, val_2_node)
  }).count();
    
  reconstructed
}

pub fn recover_val2_nodes_p (reduced_p : &Matrix<usize>, matrix_p : &Matrix<usize>, 
                             rm_nodes : &Vec<Val2Node>) -> Matrix<usize> {
  let num_rows = matrix_p.len();
  let start : Matrix<usize> = vec![(0..num_rows).collect(); num_rows];
  recover_val2_nodes(reduced_p, matrix_p, &start, rm_nodes, add_removed_node_p)
}


pub fn recover_val2_nodes_w (reduced_w : &Matrix<isize>, matrix_w : &Matrix<isize>, 
                             rm_nodes : &Vec<Val2Node>) -> Matrix<isize> {
  let num_rows = matrix_w.len();
  let start : Matrix<isize> = vec![(0..num_rows).map(|_| -1).collect(); num_rows];
  recover_val2_nodes(reduced_w, matrix_w, &start, rm_nodes, add_removed_node_w)
}


#[cfg(test)]
mod tests;
