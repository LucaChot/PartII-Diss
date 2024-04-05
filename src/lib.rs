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
use processor::{CoreInfo, Processor};


pub enum Comm {
  BROADCAST,
  FOXOTTO,
  CANNON,
}


pub struct Algorithm {
  cores_height : usize, 
  cores_width : usize,
  processor : Processor<()>
}

impl Algorithm {
  pub fn new(p_height : usize, p_width: usize) -> Self {
    Algorithm {
      cores_height : p_height,
      cores_width : p_width,
      processor : Processor::<()>::new(p_height, p_width),
    }
  }

  fn collect_c<T : Sendable>(&self, recv : Receiver<(usize, usize, Matrix<T>)>,
               matrix_c : &mut Matrix<T>) {
    let m_rows = matrix_c.len();
    let m_cols = matrix_c[0].len();
    let submatrices_dim = self.processor.get_submatrices_dim(m_rows, m_cols);

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
  
  fn parralel_mult_internal<T, F>  (&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>,
                                                  _ : F)
    -> Matrix<T> 
    where F : ParallelMatMult,
          T : Multiplicable + Sendable + 'static {
    let processor_dim = (self.cores_height, self.cores_width);
    let processor = Processor::<()>::new(self.cores_height, self.cores_width);
    let mut cores_info : VecDeque<CoreInfo<Matrix<T>>> = VecDeque::from(processor.create_taurus());
    let mut handles = Vec::with_capacity(self.cores_width * self.cores_height);
    let dim = matrix_a.len();
    // Message channel to return values from each thread
    let (main_tx, main_rx) = mpsc::channel();
    let (direct_tx, direct_rx) = mpsc::channel::<usize>();
    let (broadcast_tx, broadcast_rx) = mpsc::channel::<usize>();

    let mut submatrices_a = F::outer_setup_a(&matrix_a, &processor);
    let mut submatrices_b = F::outer_setup_b(&matrix_b, &processor);
    let mut matrix_c = T::start_c(&matrix_a);
    let mut submatrices_c = F::outer_setup_c(&matrix_c, &processor);

    for i in 0..self.cores_height {
      for j in 0..self.cores_width {
        // Assign each thread its corresponding channels
        let core_info = cores_info.pop_front().unwrap();
        // Sender for returning the results
        let result_tx = main_tx.clone();
        let dir_tx = direct_tx.clone();
        let broad_tx = broadcast_tx.clone();
        let iterations = self.cores_height;

        // Assign each threads matrix component
        let a = submatrices_a.pop_front().unwrap();
        let b = submatrices_b.pop_front().unwrap();
        let c = submatrices_c.pop_front().unwrap();

        let handle = thread::spawn(move || {
          let c = F::matrix_mult(a, b, c, iterations, &core_info);
          result_tx.send((i, j, c)).unwrap();
          let _ = dir_tx.send(core_info.core_comm.num_direct());
          let _ = broad_tx.send(core_info.core_comm.num_broadcasts());
        });
        handles.push(handle);
      }
    }
    // Ensures that channel to main thread is closed when the other threads 
    // finish
    drop(main_tx);
    drop(direct_tx);
    drop(broadcast_tx);

    self.collect_c(main_rx, &mut matrix_c);

    let mut directs = 0;
    for direct_count in direct_rx {
      directs += direct_count;
    }
    //println!("Total number of direct msgs : {}", directs);

    let mut broadcasts = 0;
    for broadcast_count in broadcast_rx {
      broadcasts += broadcast_count;
    }
    //println!("Total number of broadcasts msgs : {}", broadcasts);

    for handle in handles {
      handle.join().unwrap();
    }

    matrix_c
  }   

  fn parralel_square_internal<F, T> (&mut self, matrix_a : Matrix<T>,
                                                    outer_iterations : usize,
                                                  _ : F)
    -> Matrix<T> 
    where F : ParallelMatMult,
          T : Multiplicable + Sendable + 'static {
                                                  
    let processor_dim = (self.cores_height, self.cores_width);
    let processor = Processor::<()>::new(self.cores_height, self.cores_width);
    let mut cores_info : VecDeque<CoreInfo<Matrix<T>>> = VecDeque::from(processor.create_taurus());
    let mut handles = Vec::with_capacity(self.cores_width * self.cores_height);
    let dim = matrix_a.len();
    // Message channel to return values from each thread
    let (main_tx, main_rx) = mpsc::channel();
    let (direct_tx, direct_rx) = mpsc::channel::<usize>();
    let (broadcast_tx, broadcast_rx) = mpsc::channel::<usize>();

    let mut submatrices_a = F::outer_setup_a(&matrix_a, &processor);
    let mut submatrices_b = F::outer_setup_b(&matrix_a, &processor);
    let mut matrix_c = T::start_c(&matrix_a);
    let mut submatrices_c = F::outer_setup_c(&matrix_c, &processor);


    for i in 0..self.cores_height {
      for j in 0..self.cores_width {
        // Assign each thread its corresponding channels
        let core_info = cores_info.pop_front().unwrap();
        // Sender for returning the results
        let result_tx = main_tx.clone();
        let dir_tx = direct_tx.clone();
        let broad_tx = broadcast_tx.clone();
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
          let _ = dir_tx.send(core_info.core_comm.num_direct());
          let _ = broad_tx.send(core_info.core_comm.num_broadcasts());
        });
        handles.push(handle);
      }
    }
    // Ensures that channel to main thread is closed when the other threads 
    // finish
    drop(main_tx);
    drop(direct_tx);
    drop(broadcast_tx);

    self.collect_c(main_rx, &mut matrix_c);

    let mut directs = 0;
    for direct_count in direct_rx {
      directs += direct_count;
    }
    //println!("Total number of direct msgs : {}", directs);

    let mut broadcasts = 0;
    for broadcast_count in broadcast_rx {
      broadcasts += broadcast_count;
    }
    //println!("Total number of broadcasts msgs : {}", broadcasts);

    for handle in handles {
      handle.join().unwrap();
    }

    matrix_c
  }


  pub fn parallel_mult<T>(&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                   comm : Comm) -> Matrix<T> 
    where T : Multiplicable + Sendable + 'static {
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

  pub fn parallel_square<T>(&mut self, matrix_a : Matrix<T>,
                         iterations : usize,
                   comm : Comm) -> Matrix<T> 
    where T : Multiplicable + Sendable + 'static {
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
