use sim::types::{Msg, Matrix};

use crate::adj_matrix::Store;

#[test]
fn store_load(){
  let matrix_a: Matrix<f64> = vec![
    vec![1.0,2.0,3.0],
    vec![4.0,5.0,6.0],
    vec![7.0,8.0,9.0],
  ];

  let matrix_b: Matrix<usize> = vec![
    vec![9,8,7],
    vec![6,5,4],
    vec![3,2,1],
  ];

  //let matrix_m = Msg::zip(&matrix_a, &matrix_b);
  //let _ = matrix_m.store("src/bin/graph_io/adj_matrix/tests/test");

  let mut matrix_ml : Matrix<Msg> = Vec::new();
  let _ = matrix_ml.load("src/bin/graph_io/adj_matrix/tests/test");

  let (matrix_al, matrix_bl) = Msg::unzip(&matrix_ml);

  assert_eq!(matrix_a, matrix_al);
  assert_eq!(matrix_b, matrix_bl);
}
