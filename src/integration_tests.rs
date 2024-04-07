use crate::{Algorithm, Comm};
use crate::types::{Matrix, Msg};

#[test]
#[ignore]
fn test_hash_matrix_mult_api() {
  let mut p : Algorithm<isize> = Algorithm::new(2,2);
  
  let matrix_a: Matrix<isize> = vec![
    vec![1,2,3],
    vec![4,5,6],
    vec![7,8,9],
  ];

  let matrix_b: Matrix<isize> = vec![
    vec![9,8,7],
    vec![6,5,4],
    vec![3,2,1],
  ];

  let c = p.parallel_mult(matrix_a, matrix_b, Comm::BROADCAST);

  assert_eq!(c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_fox_otto_matrix_mult() {
  let mut p : Algorithm<isize> = Algorithm::new(2,2);
  
  let matrix_a: Matrix<isize> = vec![
    vec![1,2,3],
    vec![4,5,6],
    vec![7,8,9],
  ];

  let matrix_b: Matrix<isize> = vec![
    vec![9,8,7],
    vec![6,5,4],
    vec![3,2,1],
  ];

  let c = p.parallel_mult(matrix_a, matrix_b, Comm::FOXOTTO);

  assert_eq!(c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);
}


#[test]
#[ignore]
fn test_cannon_matrix_mult() {
  let mut p : Algorithm<isize> = Algorithm::new(2,2);
  
  let matrix_a: Matrix<isize> = vec![
    vec![1,2,3],
    vec![4,5,6],
    vec![7,8,9],
  ];

  let matrix_b: Matrix<isize> = vec![
    vec![9,8,7],
    vec![6,5,4],
    vec![3,2,1],
  ];

  let c = p.parallel_mult(matrix_a, matrix_b, Comm::CANNON);

  assert_eq!(c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_fox_otto_matrix_mult_with_reduction() {
  let mut p : Algorithm<Msg> = Algorithm::new(3,3);
  
  // P matrix
  let p_matrix: Vec<Vec<usize>> = vec![
    vec![0,0,0,0,4,5,6],
    vec![0,1,2,3,1,5,6],
    vec![0,1,2,3,4,2,2],
    vec![0,1,2,3,4,5,3],
    vec![0,1,2,3,4,5,6],
    vec![0,5,2,3,4,5,6],
    vec![0,1,2,3,4,5,6],
  ];

  // W matrix
  let w_matrix: Vec<Vec<isize>> = vec![
    vec![ 0, 6, 2, 3,-1,-1,-1],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1,-1, 0,-1,-1, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ];

  let matrix_m = Msg::zip(w_matrix, p_matrix);
  
  let iterations = f64::ceil(f64::log2(matrix_m.len() as f64)) as usize;
  let c = p.parallel_square(matrix_m, iterations, Comm::FOXOTTO);

  let (result_w, result_p) = Msg::unzip(c);

  assert_eq!(result_p, vec![
    vec![0,5,0,0,1,2,2],
    vec![0,1,2,3,1,5,6],
    vec![0,5,2,3,1,2,2],
    vec![0,1,2,3,4,5,3],
    vec![0,1,2,3,4,5,6],
    vec![0,5,2,3,1,5,6],
    vec![0,1,2,3,4,5,6],
  ]);

  assert_eq!(result_w, vec![
    vec![ 0, 5, 2, 3, 6, 4, 3],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1, 3, 0,-1, 4, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1, 2, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ]);

}
