use std::collections::VecDeque;
use std::{thread, sync::mpsc, f64};

mod broadcast;
mod processor;
mod matrix_multiplication;

use broadcast::BChannel;
use processor::fox_otto_processor;
use matrix_multiplication::fox_otto::*;

use crate::processor::{get_submatrices, get_submatrices_dim};


fn main() {
  const PROCESSOR_DIM : (usize,usize) = (3,3);
  const NUM_PROCESSORS: usize =  PROCESSOR_DIM.0 * PROCESSOR_DIM.1;
  
  // P matrix
  let p_matrix: Vec<Vec<i32>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
  ];

  // W matrix
  let w_matrix: Vec<Vec<i32>> = vec![
    vec![ 0, 6, 2, 3,-1,-1,-1],
    vec![-1, 0,-1,-1, 1,-1,-1],
    vec![-1,-1, 0,-1,-1, 2, 1],
    vec![-1,-1,-1, 0,-1,-1, 2],
    vec![-1,-1,-1,-1, 0,-1,-1],
    vec![-1, 1,-1,-1,-1, 0,-1],
    vec![-1,-1,-1,-1,-1,-1, 0],
  ];

  let m_matrix : Matrix<Msg> = w_matrix.iter().zip(p_matrix.iter())
    .map(|(w_row, p_row)| w_row.iter().zip(p_row.iter())
      .map(|(&w, &p)| Msg::new(w, p)  ).collect::<Vec<Msg>>()
    ).collect::<Matrix<Msg>>();

  // Dimensions of matrix
  let dim = m_matrix.len();
  // Number of matrix squaring that needs to be done
  let iterations = f64::ceil(f64::log2(dim as f64)) as usize;

  // Thread per element in matrix

  // Messaging channels for each thread
  let mut processors : VecDeque<(BChannel<Matrix<Msg>>, mpsc::Sender<Matrix<Msg>>, mpsc::Receiver<Matrix<Msg>>)> 
    = VecDeque::from(fox_otto_processor::<Matrix<Msg>>(PROCESSOR_DIM.0, PROCESSOR_DIM.1));

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  // Message channel to return values from each thread
  let (main_tx, main_rx) = mpsc::channel();

  let mut m_submatrices : VecDeque<Matrix<Msg>> = 
    VecDeque::from(get_submatrices(&m_matrix, PROCESSOR_DIM));


  for i in 0..PROCESSOR_DIM.0 {
    for j in 0..PROCESSOR_DIM.1 {
      // Assign each thread its corresponding channels
      let (row_broadcast, tx, rx) = processors.pop_front().unwrap();
      // Sender for returning the results
      let result_tx = main_tx.clone();

      // Processor information
      let p_info = FoxOttoProcessorInfo::new(i, j, row_broadcast, tx, rx);

      // Msg struct
      // Assign each threads matrix component
      let mut m = m_submatrices.pop_front().unwrap();

      let handle = thread::spawn(move || {
        // Square the W matrix and update P
        for _ in 0..iterations {
          m = fox_otto_matrix_mult(m.clone(), m.clone(), m.clone(), PROCESSOR_DIM.0, &p_info, singleton_pred_matrix_multiplication);
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

  let mut next_w_matrix: Vec<Vec<i32>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
  ];
  let mut next_p_matrix: Vec<Vec<i32>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
  ];

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

  dbg!(next_w_matrix);
  println!("-----------------------------");
  dbg!(next_p_matrix);

  for handle in handles {
    handle.join().unwrap();
  }

}

#[cfg(test)]
mod integration_tests;
