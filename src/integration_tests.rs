use std::collections::VecDeque;
use std::{thread, sync::mpsc};

use super::broadcast::Sendable;
use super::processor::{general_processor, CoreInfo};
use super::matrix_multiplication::fox_otto::*;
use super::matrix_multiplication::hash::*;
use super::matrix_multiplication::cannons::*;
use crate::graph_optimisation::reduction::remove_val2_nodes;
use crate::graph_optimisation::expansion::*;

use crate::processor::{get_submatrices, get_submatrices_dim};

impl Sendable for isize {}

#[test]
#[ignore]
fn test_fox_otto_matrix_mult() {
  const PROCESSOR_DIM : (usize,usize) = (2,2);
  const NUM_PROCESSORS: usize =  PROCESSOR_DIM.0 * PROCESSOR_DIM.1;
  
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

  let a_rows = matrix_a.len();
  let b_cols = matrix_b[0].len();
  
  let mut matrix_c : Matrix<isize> = vec![vec![0; b_cols]; a_rows];

  let mut cores_info : VecDeque<CoreInfo<Matrix<isize>>> 
    = VecDeque::from(general_processor::<Matrix<isize>>(PROCESSOR_DIM));

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  
  let (main_tx, main_rx) = mpsc::channel();

  let mut a_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_a, PROCESSOR_DIM));

  let mut b_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_b, PROCESSOR_DIM));

  let mut c_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_c, PROCESSOR_DIM));


  for i in 0..PROCESSOR_DIM.0 {
    for j in 0..PROCESSOR_DIM.1 {
      let core_info = cores_info.pop_front().unwrap();
      
      let result_tx = main_tx.clone();

      let a = a_submatrices.pop_front().unwrap();
      let b = b_submatrices.pop_front().unwrap();
      let mut c = c_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        c = fox_otto_matrix_mult(a, b, c, PROCESSOR_DIM.0, &core_info, singleton_matrix_multiplication);
        result_tx.send((i, j, c)).unwrap();
      });
      handles.push(handle);
    }
  }
  drop(main_tx);

  let submatrices_dim = get_submatrices_dim(PROCESSOR_DIM, (a_rows,b_cols));

  // Assign the final values to the W and P matrix
  for (i, j, c)  in main_rx {
    let index = i * PROCESSOR_DIM.1 + j;
    let submatrix_dim = submatrices_dim[index];
    for i in 0..submatrix_dim.height {
      for j in 0..submatrix_dim.width {
        matrix_c[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j];
      }
    }
  }

  dbg!(&matrix_c);

  for handle in handles {
    handle.join().unwrap();
  }

  assert_eq!(matrix_c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_hash_matrix_mult() {
  const PROCESSOR_DIM : (usize,usize) = (2,2);
  const NUM_PROCESSORS: usize =  PROCESSOR_DIM.0 * PROCESSOR_DIM.1;
  
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

  let a_rows = matrix_a.len();
  let b_cols = matrix_b[0].len();
  
  let mut matrix_c : Matrix<isize> = vec![vec![0; b_cols]; a_rows];

  let mut cores_info : VecDeque<CoreInfo<Matrix<isize>>> 
    = VecDeque::from(general_processor::<Matrix<isize>>(PROCESSOR_DIM));

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  
  let (main_tx, main_rx) = mpsc::channel();

  let mut a_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_a, PROCESSOR_DIM));

  let mut b_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_b, PROCESSOR_DIM));

  let mut c_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_c, PROCESSOR_DIM));


  for i in 0..PROCESSOR_DIM.0 {
    for j in 0..PROCESSOR_DIM.1 {
      let core_info = cores_info.pop_front().unwrap();
      let result_tx = main_tx.clone();


      let a = a_submatrices.pop_front().unwrap();
      let b = b_submatrices.pop_front().unwrap();
      let mut c = c_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        c = hash_matrix_mult(a, b, c, PROCESSOR_DIM.0, &core_info, singleton_matrix_multiplication);
        result_tx.send((i, j, c)).unwrap();
      });
      handles.push(handle);
    }
  }
  drop(main_tx);

  let submatrices_dim = get_submatrices_dim(PROCESSOR_DIM, (a_rows,b_cols));

  // Assign the final values to the W and P matrix
  for (i, j, c)  in main_rx {
    let index = i * PROCESSOR_DIM.1 + j;
    let submatrix_dim = submatrices_dim[index];
    for i in 0..submatrix_dim.height {
      for j in 0..submatrix_dim.width {
        matrix_c[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j];
      }
    }
  }

  dbg!(&matrix_c);

  for handle in handles {
    handle.join().unwrap();
  }

  assert_eq!(matrix_c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_cannon_matrix_mult() {
  const PROCESSOR_DIM : (usize,usize) = (2,2);
  const NUM_PROCESSORS: usize =  PROCESSOR_DIM.0 * PROCESSOR_DIM.1;
  
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

  let a_rows = matrix_a.len();
  let b_cols = matrix_b[0].len();
  
  let mut matrix_c : Matrix<isize> = vec![vec![0; b_cols]; a_rows];

  let mut cores_info : VecDeque<CoreInfo<Matrix<isize>>> 
    = VecDeque::from(general_processor::<Matrix<isize>>(PROCESSOR_DIM));

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  
  let (main_tx, main_rx) = mpsc::channel();

  let mut a_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_a, PROCESSOR_DIM));

  a_submatrices = cannon_setup_a(a_submatrices, PROCESSOR_DIM);

  let mut b_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_b, PROCESSOR_DIM));

  b_submatrices = cannon_setup_b(b_submatrices, PROCESSOR_DIM);

  let mut c_submatrices : VecDeque<Matrix<isize>> = 
    VecDeque::from(get_submatrices(&matrix_c, PROCESSOR_DIM));


  for i in 0..PROCESSOR_DIM.0 {
    for j in 0..PROCESSOR_DIM.1 {
      let core_info = cores_info.pop_front().unwrap();
      
      let result_tx = main_tx.clone();

      let a = a_submatrices.pop_front().unwrap();
      let b = b_submatrices.pop_front().unwrap();
      let mut c = c_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        c = cannon_matrix_mult(a, b, c, PROCESSOR_DIM.0, &core_info, singleton_matrix_multiplication);
        result_tx.send((i, j, c)).unwrap();
      });
      handles.push(handle);
    }
  }
  drop(main_tx);

  let submatrices_dim = get_submatrices_dim(PROCESSOR_DIM, (a_rows,b_cols));

  // Assign the final values to the W and P matrix
  for (i, j, c)  in main_rx {
    let index = i * PROCESSOR_DIM.1 + j;
    let submatrix_dim = submatrices_dim[index];
    for i in 0..submatrix_dim.height {
      for j in 0..submatrix_dim.width {
        matrix_c[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j];
      }
    }
  }

  dbg!(&matrix_c);

  for handle in handles {
    handle.join().unwrap();
  }

  assert_eq!(matrix_c, vec![
    vec![30,24,18],
    vec![84,69,54],
    vec![138,114,90]
  ]);

}

#[test]
#[ignore]
fn test_fox_otto_matrix_mult_with_reduction() {
  const PROCESSOR_DIM : (usize,usize) = (3,3);
  const NUM_PROCESSORS: usize =  PROCESSOR_DIM.0 * PROCESSOR_DIM.1;
  
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

  let (reduced_w, reduced_p, removed_val2_nodes) = remove_val2_nodes(&w_matrix, &p_matrix);
  

  let m_matrix : Matrix<Msg> = reduced_w.iter().zip(reduced_p.iter())
    .map(|(w_row, p_row)| w_row.iter().zip(p_row.iter())
      .map(|(&w, &p)| Msg::new(w, p)  ).collect::<Vec<Msg>>()
    ).collect::<Matrix<Msg>>();

  // Dimensions of matrix
  let dim = m_matrix.len();
  // Number of matrix squaring that needs to be done
  let iterations = f64::ceil(f64::log2(dim as f64)) as usize;

  // Thread per element in matrix

  // Messaging channels for each thread
  let mut cores_info : VecDeque<CoreInfo<Matrix<Msg>>> 
    = VecDeque::from(general_processor::<Matrix<Msg>>(PROCESSOR_DIM));

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  // Message channel to return values from each thread
  let (main_tx, main_rx) = mpsc::channel();

  let mut m_submatrices : VecDeque<Matrix<Msg>> = 
    VecDeque::from(get_submatrices(&m_matrix, PROCESSOR_DIM));


  for i in 0..PROCESSOR_DIM.0 {
    for j in 0..PROCESSOR_DIM.1 {
      // Assign each thread its corresponding channels
      let core_info = cores_info.pop_front().unwrap();
      // Sender for returning the results
      let result_tx = main_tx.clone();

      // Msg struct
      // Assign each threads matrix component
      let mut m = m_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        // Square the W matrix and update P
        for _ in 0..iterations {
          m = fox_otto_matrix_mult(m.clone(), m.clone(), m.clone(), PROCESSOR_DIM.0, &core_info, singleton_pred_matrix_multiplication);
        }
        // Return the final values for the W and P matrix as well as the
        // index of the core so that main thread knows the values corresponding
        // location
        result_tx.send((i, j, m)).unwrap();
      });
      handles.push(handle);
    }
  }

  // Ensures that channel to main thread is closed when the other threads 
  // finish
  drop(main_tx);

  let submatrices_dim = get_submatrices_dim(PROCESSOR_DIM, (dim,dim));

  let mut next_w_matrix: Vec<Vec<isize>> = vec![(0..dim).map(|_| -1).collect();dim];
  let mut next_p_matrix: Vec<Vec<usize>> = vec![(0..dim).collect();dim];
  // Assign the final values to the W and P matrix
  for (i, j, c)  in main_rx {
    let index = i * PROCESSOR_DIM.1 + j;
    let submatrix_dim = submatrices_dim[index];
    for i in 0..submatrix_dim.height {
      for j in 0..submatrix_dim.width {
        next_w_matrix[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j].get_w();
        next_p_matrix[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j].get_p();
      }
    }
  }

  let expanded_p = recover_val2_nodes_p(&next_p_matrix, &p_matrix, &removed_val2_nodes);
  let expanded_w = recover_val2_nodes_w(&next_w_matrix, &w_matrix, &removed_val2_nodes);

  for handle in handles {
    handle.join().unwrap();
  }

  assert_eq!(expanded_p, vec![
    vec![0,5,0,0,1,2,2],
    vec![0,1,2,3,1,5,6],
    vec![0,5,2,3,1,2,2],
    vec![0,1,2,3,4,5,3],
    vec![0,1,2,3,4,5,6],
    vec![0,5,2,3,1,5,6],
    vec![0,1,2,3,4,5,6],
  ]);

  assert_eq!(expanded_w, vec![
    vec![ 0, 5, 2, 3, 6, 4, 3],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1, 3, 0,-1, 4, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1, 2, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ]);

}
