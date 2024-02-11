use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::{thread, sync::mpsc};

mod broadcast;
mod processor;
mod matrix_multiplication;
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


  pub fn parallel_mult(&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
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

  pub fn parallel_square(&mut self, matrix_a : Matrix<T>,
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

#[cfg(test)]
mod integration_tests;
