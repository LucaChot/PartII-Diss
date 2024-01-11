use std::{vec, panic};

use crate::graph_optimisation::is_val2_nodes;

use super::*;

#[test]
fn test_valency_2_no_links(){
  let links_in : Vec<usize> = vec![0,0,0];
  let links_out : Vec<usize> = vec![0,1,2];
  assert_eq!(is_val2_nodes(0, &links_in, &links_out), None);
}

#[test]
fn test_valency_2_only_in(){
  let links_in : Vec<usize> = vec![0,1,0];
  let links_out : Vec<usize> = vec![0,1,2];
  assert_eq!(is_val2_nodes(0, &links_in, &links_out), None);
}

#[test]
fn fest_valency_2_only_out(){
  let links_in : Vec<usize> = vec![0,0,0];
  let links_out : Vec<usize> = vec![0,0,2];
  assert_eq!(is_val2_nodes(0, &links_in, &links_out), None);
}

#[test]
fn test_valency_2_true(){
  let links_in : Vec<usize> = vec![0,0,2];
  let links_out : Vec<usize> = vec![0,0,2];
  assert_eq!(is_val2_nodes(0, &links_in, &links_out), Some((2,1)));
}

#[test]
fn test_remove_val2_nodes_linear(){
  let matrix_w = vec![
    vec![0,1,-1],
    vec![-1,0,1],
    vec![-1,-1,0]
  ];
  let matrix_p = vec![
    vec![0,0,2],
    vec![0,1,1],
    vec![0,1,2]
  ];
  let result = new_remove_val2_nodes(&matrix_w, &matrix_p);
  assert_eq!(result.0, vec![
    vec![0,2],
    vec![-1,0]
  ]);
  assert_eq!(result.1, vec![
    vec![0,0],
    vec![0,2]
  ]);
}

#[test]
fn test_remove_val2_nodes_non_linear(){
  let matrix_w = vec![
    vec![ 0, 6, 2, 3,-1,-1,-1],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1,-1, 0,-1,-1, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ];
  let matrix_p = vec![
    vec![0,0,0,0,4,5,6],
    vec![0,1,2,3,1,5,6],
    vec![0,1,2,3,4,2,2],
    vec![0,1,2,3,4,5,3],
    vec![0,1,2,3,4,5,6],
    vec![0,5,2,3,4,5,6],
    vec![0,1,2,3,4,5,6],
  ];
  let result = new_remove_val2_nodes(&matrix_w, &matrix_p);
  assert_eq!(result.0, vec![
    vec![ 0, 6, 2,-1, 5],
    vec![-1, 0,-1, 1,-1],
    vec![-1, 3, 0,-1, 1],
    vec![-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1, 0],
  ]);
  assert_eq!(result.1, vec![
    vec![0,0,0,4,0],
    vec![0,1,2,1,6],
    vec![0,2,2,4,2],
    vec![0,1,2,4,6],
    vec![0,1,2,4,6],
  ]);
}

#[test]
fn test_remove_val2_nodes_original_matrix_untouched(){
  let matrix_w = vec![
    vec![0,1,-1],
    vec![-1,0,1],
    vec![-1,-1,0]
  ];
  let matrix_p = vec![
    vec![0,0,2],
    vec![0,1,1],
    vec![0,1,2]
  ];
  let _ = new_remove_val2_nodes(&matrix_w, &matrix_p);
  assert_eq!(matrix_w, vec![
    vec![0,1,-1],
    vec![-1,0,1],
    vec![-1,-1,0]
  ]);
  assert_eq!(matrix_p, vec![
    vec![0,0,2],
    vec![0,1,1],
    vec![0,1,2]
  ]);
}

#[test]
fn test_recover_val2_nodes_p_simple() {
  let matrix_p = vec![
    vec![0,0,2,3],
    vec![0,1,1,3],
    vec![0,1,2,2],
    vec![0,1,2,3],
  ];
  let reduced_p = vec![
    vec![0,0],
    vec![0,3],
  ];
  let removed_nodes = vec![Val2Node::new(1,0,1,2,1), Val2Node::new(2,0,1,3,1)];
  let result = recover_val2_nodes_p(&reduced_p, &matrix_p, &removed_nodes);
  assert_eq!(result, vec![
    vec![0,0,1,2],
    vec![0,1,1,2],
    vec![0,1,2,2],
    vec![0,1,2,3],
  ]);
  dbg!(result);
  //panic!();
}

#[test]
fn test_recover_val2_nodes_p_simple_diff_order() {
  let matrix_p = vec![
    vec![0,0,2,3],
    vec![0,1,1,3],
    vec![0,1,2,2],
    vec![0,1,2,3],
  ];
  let reduced_p = vec![
    vec![0,0],
    vec![0,3],
  ];
  let removed_nodes = vec![Val2Node::new(2,1,1,3,1), Val2Node::new(1,0,1,3,1)];
  let result = recover_val2_nodes_p(&reduced_p, &matrix_p, &removed_nodes);
  assert_eq!(result, vec![
    vec![0,0,1,2],
    vec![0,1,1,2],
    vec![0,1,2,2],
    vec![0,1,2,3],
  ]);
  dbg!(result);
  //panic!();
}

#[test]
fn test_recover_val2_nodes_p_loop() {
  let matrix_p = vec![
    vec![0,0,2,3],
    vec![0,1,1,3],
    vec![0,1,2,2],
    vec![3,1,2,3],
  ];
  let reduced_p = vec![
    vec![0,0],
    vec![2,2],
  ];
  let removed_nodes = vec![Val2Node::new(3,2,1,0,1), Val2Node::new(1,0,1,2,1)];
  let result = recover_val2_nodes_p(&reduced_p, &matrix_p, &removed_nodes);
  dbg!(&result);
  assert_eq!(result, vec![
    vec![0,0,1,2],
    vec![3,1,1,2],
    vec![3,0,2,2],
    vec![3,0,1,3],
  ]);
  //panic!();
}

#[test]
fn test_recover_val2_nodes_p_loop_diff_order() {
  let matrix_p = vec![
    vec![0,0,2,3],
    vec![0,1,1,3],
    vec![0,1,2,2],
    vec![3,1,2,3],
  ];
  let reduced_p = vec![
    vec![0,0],
    vec![3,3],
  ];
  let removed_nodes = vec![Val2Node::new(1,0,1,2,1), Val2Node::new(2,0,1,3,1)];
  let result = recover_val2_nodes_p(&reduced_p, &matrix_p, &removed_nodes);
  dbg!(&result);
  assert_eq!(result, vec![
    vec![0,0,1,2],
    vec![3,1,1,2],
    vec![3,0,2,2],
    vec![3,0,1,3],
  ]);
  //panic!();
}

#[test]
fn test_recover_val2_nodes_p_complex() {
  let matrix_p = vec![
    vec![0,0,0,0,4,5,6],
    vec![0,1,2,3,1,5,6],
    vec![0,1,2,3,4,2,2],
    vec![0,1,2,3,4,5,3],
    vec![0,1,2,3,4,5,6],
    vec![0,5,2,3,4,5,6],
    vec![0,1,2,3,4,5,6],
  ];
  let reduced_p = vec![
    vec![0,2,0,1,2],
    vec![0,1,2,1,6],
    vec![0,2,2,1,2],
    vec![0,1,2,4,6],
    vec![0,1,2,4,6],
  ];
  let removed_nodes = vec![Val2Node::new(3,0,3,6,2), Val2Node::new(5,2,2,1,1)];
  let result = recover_val2_nodes_p(&reduced_p, &matrix_p, &removed_nodes);
  assert_eq!(result, vec![
    vec![0,5,0,0,1,2,2],
    vec![0,1,2,3,1,5,6],
    vec![0,5,2,3,1,2,2],
    vec![0,1,2,3,4,5,3],
    vec![0,1,2,3,4,5,6],
    vec![0,5,2,3,1,5,6],
    vec![0,1,2,3,4,5,6],
  ]);
  dbg!(result);
  //panic!();
}

//------------------------------------------------------------ 

#[test]
fn test_recover_val2_nodes_w_simple() {
  let matrix_w = vec![
    vec![ 0, 1,-1,-1],
    vec![-1, 0, 1,-1],
    vec![-1,-1, 0, 1],
    vec![-1,-1,-1, 0],
  ];
  let reduced_w = vec![
    vec![ 0, 3],
    vec![-1, 0],
  ];
  let removed_nodes = vec![Val2Node::new(1,0,1,2,1), Val2Node::new(2,0,2,3,1)];
  let result = recover_val2_nodes_w(&reduced_w, &matrix_w, &removed_nodes);
  assert_eq!(result, vec![
    vec![ 0, 1, 2, 3],
    vec![-1, 0, 1, 2],
    vec![-1,-1, 0, 1],
    vec![-1,-1,-1, 0],
  ]);
}

#[test]
fn test_recover_val2_nodes_w_simple_diff_order() {
  let matrix_w = vec![
    vec![ 0, 1,-1,-1],
    vec![-1, 0, 1,-1],
    vec![-1,-1, 0, 1],
    vec![-1,-1,-1, 0],
  ];
  let reduced_w = vec![
    vec![ 0, 3],
    vec![-1, 0],
  ];
  let removed_nodes = vec![Val2Node::new(2,1,1,3,1), Val2Node::new(1,0,1,3,2)];
  let result = recover_val2_nodes_w(&reduced_w, &matrix_w, &removed_nodes);
  assert_eq!(result, vec![
    vec![ 0, 1, 2, 3],
    vec![-1, 0, 1, 2],
    vec![-1,-1, 0, 1],
    vec![-1,-1,-1, 0],
  ]);
}

#[test]
fn test_recover_val2_nodes_w_loop() {
  let matrix_w = vec![
    vec![ 0, 1,-1,-1],
    vec![-1, 0, 1,-1],
    vec![-1,-1, 0, 1],
    vec![ 1,-1,-1, 0],
  ];
  let reduced_w = vec![
    vec![0,2],
    vec![2,0],
  ];
  let removed_nodes = vec![Val2Node::new(3,2,1,0,1), Val2Node::new(1,0,1,2,1)];
  let result = recover_val2_nodes_w(&reduced_w, &matrix_w, &removed_nodes);
  assert_eq!(result, vec![
    vec![ 0, 1, 2, 3],
    vec![ 3, 0, 1, 2],
    vec![ 2, 3, 0, 1],
    vec![ 1, 2, 3, 0],
  ]);
  //panic!();
}

#[test]
fn test_recover_val2_nodes_w_loop_diff_order() {
  let matrix_w = vec![
    vec![ 0, 1,-1,-1],
    vec![-1, 0, 1,-1],
    vec![-1,-1, 0, 1],
    vec![ 1,-1,-1, 0],
  ];
  let reduced_w = vec![
    vec![0,3],
    vec![1,0],
  ];
  let removed_nodes = vec![Val2Node::new(1,0,1,2,1), Val2Node::new(2,0,2,3,1)];
  let result = recover_val2_nodes_w(&reduced_w, &matrix_w, &removed_nodes);
  assert_eq!(result, vec![
    vec![ 0, 1, 2, 3],
    vec![ 3, 0, 1, 2],
    vec![ 2, 3, 0, 1],
    vec![ 1, 2, 3, 0],
  ]);
  //panic!();
}
#[test]
fn test_recover_val2_nodes_w_complex() {
  let matrix_w = vec![
    vec![ 0, 6, 2, 3,-1,-1,-1],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1,-1, 0,-1,-1, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ];
  let reduced_w = vec![
    vec![ 0, 5, 2, 6, 3],
    vec![-1, 0,-1, 1,-1],
    vec![-1, 3, 0, 4, 1],
    vec![-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1, 0],
  ];
  let removed_nodes = vec![Val2Node::new(3,0,3,6,2), Val2Node::new(5,2,2,1,1)];
  let result = recover_val2_nodes_w(&reduced_w, &matrix_w, &removed_nodes);
  assert_eq!(result, vec![
    vec![ 0, 5, 2, 3, 6, 4, 3],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1, 3, 0,-1, 4, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1, 2, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ]);
}
