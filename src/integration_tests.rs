use crate::matmul::{MatMul, comm_method::{Hash, FoxOtto, Cannon}};
use crate::processor::{Processor, TaurusNetworkBuilder};
use crate::types::{Matrix, Msg};

#[test]
#[ignore]
fn test_hash_matrix_mult_api() {
  let network_builder = TaurusNetworkBuilder::new(0, 1, 0);
  let mut processor = Processor::new(2,2, Box::new(network_builder));
  let mut p : MatMul<isize> = MatMul::new(&mut processor);
  
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

  let c = p.parallel_mult::<Hash>(matrix_a, matrix_b);

  assert_eq!(c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_fox_otto_matrix_mult() {
  let network_builder = TaurusNetworkBuilder::new(0, 1, 0);
  let mut processor = Processor::new(2,2, Box::new(network_builder));
  let mut p : MatMul<isize> = MatMul::new(&mut processor);
  
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

  let c = p.parallel_mult::<FoxOtto>(matrix_a, matrix_b);

  assert_eq!(c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);
}


#[test]
#[ignore]
fn test_cannon_matrix_mult() {
  let network_builder = TaurusNetworkBuilder::new(0, 1, 0);
  let mut processor = Processor::new(2,2, Box::new(network_builder));
  let mut p : MatMul<isize> = MatMul::new(&mut processor);
  
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

  let c = p.parallel_mult::<Cannon>(matrix_a, matrix_b);

  assert_eq!(c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_fox_otto_matrix_mult_with_reduction() {
  let network_builder = TaurusNetworkBuilder::new(0, 1, 0);
  let mut processor = Processor::new(3,3, Box::new(network_builder));
  let mut p : MatMul<Msg> = MatMul::new(&mut processor);
  
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
  let w_matrix: Vec<Vec<f64>> = vec![
    vec![ 0.0, 6.0, 2.0, 3.0,-1.0,-1.0,-1.0],
    vec![-1.0, 0.0,-1.0,-1.0, 1.0,-1.0,-1.0],
    vec![-1.0,-1.0, 0.0,-1.0,-1.0, 2.0, 1.0],
    vec![-1.0,-1.0,-1.0, 0.0,-1.0,-1.0, 2.0],
    vec![-1.0,-1.0,-1.0,-1.0, 0.0,-1.0,-1.0],
    vec![-1.0, 1.0,-1.0,-1.0,-1.0, 0.0,-1.0],
    vec![-1.0,-1.0,-1.0,-1.0,-1.0,-1.0, 0.0],
  ];

  let matrix_m = Msg::zip(&w_matrix, &p_matrix);
  
  let iterations = f64::ceil(f64::log2(matrix_m.len() as f64)) as usize;
  let c = p.parallel_square::<FoxOtto>(matrix_m, iterations);

  let (result_w, result_p) = Msg::unzip(&c);

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
    vec![ 0.0, 5.0, 2.0, 3.0, 6.0, 4.0, 3.0],
    vec![-1.0, 0.0,-1.0,-1.0, 1.0,-1.0,-1.0],
    vec![-1.0, 3.0, 0.0,-1.0, 4.0, 2.0, 1.0],
    vec![-1.0,-1.0,-1.0, 0.0,-1.0,-1.0, 2.0],
    vec![-1.0,-1.0,-1.0,-1.0, 0.0,-1.0,-1.0],
    vec![-1.0, 1.0,-1.0,-1.0, 2.0, 0.0,-1.0],
    vec![-1.0,-1.0,-1.0,-1.0,-1.0,-1.0, 0.0],
  ]);

}
