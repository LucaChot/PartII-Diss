use std::collections::VecDeque;
use std::{thread, sync::mpsc};

use super::broadcast::{Sendable, BChannel};
use super::processor::{fox_otto_processor, hashtag_processor};
use super::matrix_multiplication::fox_otto::*;
use super::matrix_multiplication::hash::*;

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

  let mut processors : VecDeque<(BChannel<Matrix<_>>, mpsc::Sender<Matrix<_>>, mpsc::Receiver<Matrix<_>>)> 
    = VecDeque::from(fox_otto_processor::<Matrix<isize>>(PROCESSOR_DIM.0, PROCESSOR_DIM.1));

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
      let (row_broadcast, tx, rx) = processors.pop_front().unwrap();
      
      let result_tx = main_tx.clone();

      let p_info = FoxOttoProcessorInfo::new(i, j, row_broadcast, tx, rx);

      let a = a_submatrices.pop_front().unwrap();
      let b = b_submatrices.pop_front().unwrap();
      let mut c = c_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        c = fox_otto_matrix_mult(a, b, c, PROCESSOR_DIM.0, &p_info, singleton_matrix_multiplication);
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

  let mut processors : VecDeque<Vec<BChannel<Matrix<_>>>> 
    = VecDeque::from(hashtag_processor::<Matrix<isize>>(PROCESSOR_DIM.0, PROCESSOR_DIM.1));

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
      let mut broadcasts = processors.pop_front().unwrap();
      let col_broadcast = broadcasts.pop().unwrap();
      let row_broadcast = broadcasts.pop().unwrap();
      
      let result_tx = main_tx.clone();

      let p_info = ProcessorInfo::new(i, j, row_broadcast, col_broadcast);

      let a = a_submatrices.pop_front().unwrap();
      let b = b_submatrices.pop_front().unwrap();
      let mut c = c_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        c = thread_matrix_mult(a, b, c, PROCESSOR_DIM.0, &p_info, singleton_matrix_multiplication);
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
