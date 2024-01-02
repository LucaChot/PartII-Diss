use std::{vec, panic};

use crate::graph_optimisation::is_val2_nodes;

use super::remove_val2_nodes;

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
  let mut result = remove_val2_nodes(matrix_w, matrix_p);
  result = dbg!(result);
  assert_eq!(result.0, vec![
    vec![0,2],
    vec![-1,0]
  ]);
  assert_eq!(result.1, vec![
    vec![0,0],
    vec![0,1]
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
  let result = remove_val2_nodes(matrix_w, matrix_p);
  assert_eq!(result.0, vec![
    vec![ 0, 6, 2,-1, 5],
    vec![-1, 0,-1, 1,-1],
    vec![-1, 3, 0,-1, 1],
    vec![-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1, 0],
  ]);
  assert_eq!(result.1, vec![
    vec![0,0,0,3,0],
    vec![0,1,2,1,4],
    vec![0,2,2,3,2],
    vec![0,1,2,3,4],
    vec![0,1,2,3,4],
  ]);
}
