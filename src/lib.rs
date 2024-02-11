use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::{thread, sync::mpsc};

mod broadcast;
mod processor;
mod matrix_multiplication;
mod graph_optimisation;
mod types;

pub use broadcast::Sendable;
pub use types::{Matrix, Msg};
use matrix_multiplication::*;
pub use matrix_multiplication::Multiplicable;
use processor::{CoreInfo,general_processor};

use crate::processor::get_submatrices_dim;

pub enum Comm {
  BROADCAST,
  FOXOTTO,
  CANNON,
}


pub struct Processor<T : Multiplicable + Sendable> {
  cores_height : usize, 
  cores_width : usize,
  cores_info : VecDeque<CoreInfo<Matrix<T>>>,
}

impl<T : Multiplicable + Sendable + 'static> Processor<T>{
  pub fn new(p_height : usize, p_width: usize) -> Self {
    Processor {
      cores_height : p_height,
      cores_width : p_width,
      cores_info : VecDeque::from(general_processor::<Matrix<T>>((p_height, p_width))),
    }
  }

  fn collect_c(&self, recv : Receiver<(usize, usize, Matrix<T>)>,
               matrix_c : &mut Matrix<T>, c_dim : (usize,usize)) {
    let submatrices_dim = get_submatrices_dim((self.cores_height, self.cores_width),
    c_dim);

    // Assign the final values to the W and P matrix
    for (i, j, c)  in recv {
      let index = i * self.cores_width + j;
      let submatrix_dim = submatrices_dim[index];
      for i in 0..submatrix_dim.height {
        for j in 0..submatrix_dim.width {
          matrix_c[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j].clone();
        }
      }
    }
  }
  
  fn parralel_mult_internal<F : ParallelMatMult> (&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>,
                                                  _ : F)
    -> Matrix<T> {
    let processor_dim = (self.cores_height, self.cores_width);
    let mut handles = Vec::with_capacity(self.cores_width * self.cores_height);
    let dim = matrix_a.len();
    // Message channel to return values from each thread
    let (main_tx, main_rx) = mpsc::channel();

    let mut submatrices_a = F::outer_setup_a(&matrix_a, processor_dim);
    let mut submatrices_b = F::outer_setup_b(&matrix_b, processor_dim);
    let mut matrix_c = T::start_c(&matrix_a);
    let mut submatrices_c = F::outer_setup_c(&matrix_c, processor_dim);

    for i in 0..self.cores_height {
      for j in 0..self.cores_width {
        // Assign each thread its corresponding channels
        let core_info = self.cores_info.pop_front().unwrap();
        // Sender for returning the results
        let result_tx = main_tx.clone();
        let iterations = self.cores_height;

        // Assign each threads matrix component
        let a = submatrices_a.pop_front().unwrap();
        let b = submatrices_b.pop_front().unwrap();
        let c = submatrices_c.pop_front().unwrap();

        let handle = thread::spawn(move || {
          let c = F::matrix_mult(a, b, c, iterations, &core_info);
          result_tx.send((i, j, c)).unwrap();
        });
        handles.push(handle);
      }
    }
    // Ensures that channel to main thread is closed when the other threads 
    // finish
    drop(main_tx);

    self.collect_c(main_rx, &mut matrix_c, (dim,dim));

    for handle in handles {
      handle.join().unwrap();
    }

    matrix_c
  }   

  fn parralel_square_internal<F : ParallelMatMult> (&mut self, matrix_a : Matrix<T>,
                                                    outer_iterations : usize,
                                                  _ : F)
    -> Matrix<T> {
                                                  
    let processor_dim = (self.cores_height, self.cores_width);
    let mut handles = Vec::with_capacity(self.cores_width * self.cores_height);
    let dim = matrix_a.len();
    // Message channel to return values from each thread
    let (main_tx, main_rx) = mpsc::channel();

    let mut submatrices_a = F::outer_setup_a(&matrix_a, processor_dim);
    let mut submatrices_b = F::outer_setup_b(&matrix_a, processor_dim);
    let mut matrix_c = T::start_c(&matrix_a);
    let mut submatrices_c = F::outer_setup_c(&matrix_c, processor_dim);


    for i in 0..self.cores_height {
      for j in 0..self.cores_width {
        // Assign each thread its corresponding channels
        let core_info = self.cores_info.pop_front().unwrap();
        // Sender for returning the results
        let result_tx = main_tx.clone();
        let inner_iterations = self.cores_height;

        // Assign each threads matrix component
        let mut a = submatrices_a.pop_front().unwrap();
        let mut b = submatrices_b.pop_front().unwrap();
        let mut c = submatrices_c.pop_front().unwrap();

        let handle = thread::spawn(move || {
          for _ in 0..outer_iterations{
            c = F::matrix_mult(a, b, c, inner_iterations, &core_info);
            a = F::inner_setup_a(c.clone(), &core_info);
            b = F::inner_setup_b(c.clone(), &core_info);
          }
          result_tx.send((i, j, c)).unwrap();
        });
        handles.push(handle);
      }
    }
    // Ensures that channel to main thread is closed when the other threads 
    // finish
    drop(main_tx);

    self.collect_c(main_rx, &mut matrix_c, (dim,dim));

    for handle in handles {
      handle.join().unwrap();
    }

    matrix_c
  }


  pub fn parralel_mult(&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                   comm : Comm) -> Matrix<T> {
    match comm {
      Comm::BROADCAST => self.parralel_mult_internal(matrix_a,
                                          matrix_b,
                                          Hash),
      Comm::FOXOTTO => self.parralel_mult_internal(matrix_a,
                                          matrix_b, 
                                          FoxOtto),

      Comm::CANNON => self.parralel_mult_internal(matrix_a,
                                          matrix_b, 
                                          Cannon),
    }
  }

  pub fn parralel_square(&mut self, matrix_a : Matrix<T>,
                         iterations : usize,
                   comm : Comm) -> Matrix<T> {
    match comm {
      Comm::BROADCAST => self.parralel_square_internal(matrix_a,
                                                       iterations,
                                          Hash),
      Comm::FOXOTTO => self.parralel_square_internal(matrix_a,
                                                     iterations,
                                          FoxOtto),

      Comm::CANNON => self.parralel_square_internal(matrix_a,
                                                    iterations,
                                          Cannon),
    }
  }
}


/*
fn main() {
  const PROCESSOR_DIM : (usize,usize) = (3,3);
  const NUM_PROCESSORS: usize =  PROCESSOR_DIM.0 * PROCESSOR_DIM.1;
  
  // P matrix
  let p_matrix: Vec<Vec<usize>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
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
  let mut cores_info : VecDeque<CoreInfo<Matrix<Msg>>> 
    = VecDeque::from(general_processor::<Matrix<Msg>>(PROCESSOR_DIM));

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  // Message channel to return values from each thread
  let (main_tx, main_rx) = mpsc::channel();

  let mut m_submatrices = FoxOtto::setup_a(&m_matrix, PROCESSOR_DIM);


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
          m = FoxOtto::matrix_mult(m.clone(), m.clone(), m.clone(), PROCESSOR_DIM.0, &core_info, singleton_pred_matrix_multiplication);
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

  let mut next_w_matrix: Vec<Vec<isize>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
  ];
  let mut next_p_matrix: Vec<Vec<usize>> = vec![
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
*/

#[cfg(test)]
mod integration_tests;
