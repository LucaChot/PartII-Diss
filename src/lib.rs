use std::collections::VecDeque;

mod broadcast;
mod processor;
mod matrix_multiplication;
mod types;

pub use broadcast::Sendable;
pub use types::{Matrix, Msg};
use matrix_multiplication::*;
pub use matrix_multiplication::{Multiplicable, FoxOtto, Cannon, Hash};
use processor::{TaurusCoreInfo, Processor, TaurusNetworkBuilder, get_submatrices_dim};
pub use processor::{NetworkBuilder, CoreInfo};

pub struct MatMul<T> 
where T : Multiplicable + Sendable + 'static {
  cores_height : usize, 
  cores_width : usize,
  processor : Processor<(usize, usize, Matrix<T>),Matrix<T>, TaurusCoreInfo<Matrix<T>>>
}

impl<T> MatMul<T> 
where T : Multiplicable + Sendable + 'static {
  pub fn new(p_height : usize, p_width: usize) -> Self {
    MatMul {
      cores_height : p_height,
      cores_width : p_width,
      processor : Processor::new(p_height, p_width, Box::new(TaurusNetworkBuilder {})),
    }
  }

  fn collect_c(&self, core_results : &Vec<(usize, usize, Matrix<T>)>,
               matrix_c : &mut Matrix<T>) {
    let m_rows = matrix_c.len();
    let m_cols = matrix_c[0].len();
    let submatrices_dim = get_submatrices_dim(self.processor.rows, self.processor.cols, m_rows, m_cols);

    // Assign the final values to the W and P matrix
    for (i, j, c)  in core_results {
      let index = i * self.cores_width + j;
      let submatrix_dim = submatrices_dim[index];
      for i in 0..submatrix_dim.height {
        for j in 0..submatrix_dim.width {
          matrix_c[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j].clone();
        }
      }
    }
  }
  
  pub fn parallel_mult<F>  (&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>)
    -> Matrix<T> 
    where F : ParallelMatMult {
    let mut cores_info : VecDeque<TaurusCoreInfo<Matrix<T>>> = VecDeque::from(self.processor.build_network());

    let mut submatrices_a = F::outer_setup_a(self.processor.rows, self.processor.cols, &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows, self.processor.cols, &matrix_b);
    let mut matrix_c = T::neutral_element(matrix_a.len(), matrix_b[0].len());
    let mut submatrices_c = F::outer_setup_c(self.processor.rows, self.processor.cols, &matrix_c);

    for i in 0..self.cores_height {
      for j in 0..self.cores_width {
        // Assign each thread its corresponding channels
        let core_info = cores_info.pop_front().unwrap();
        // Sender for returning the results
        let iterations = self.cores_height;

        // Assign each threads matrix component
        let a = submatrices_a.pop_front().unwrap();
        let b = submatrices_b.pop_front().unwrap();
        let c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut TaurusCoreInfo<Vec<Vec<T>>>| {
          let c = F::matrix_mult(a, b, c, iterations, core_info);
          (i,j,c)
        };

        self.processor.run_core(core_function, core_info);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    self.processor.display_processor_time();
    matrix_c
  }   

  pub fn parallel_square<F> (&mut self, matrix_a : Matrix<T>, outer_iterations : usize)
    -> Matrix<T> 
    where F : ParallelMatMult {
    let mut cores_info : VecDeque<TaurusCoreInfo<Matrix<T>>> = VecDeque::from(self.processor.build_network());

    let mut submatrices_a = F::outer_setup_a(self.processor.rows, self.processor.cols, &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows, self.processor.cols, &matrix_a);
    let mut matrix_c = T::neutral_element(matrix_a.len(), matrix_a[0].len());
    let mut submatrices_c = F::outer_setup_c(self.processor.rows, self.processor.cols, &matrix_c);

    for i in 0..self.cores_height {
      for j in 0..self.cores_width {
        // Assign each thread its corresponding channels
        let core_info = cores_info.pop_front().unwrap();
        // Sender for returning the results
        let inner_iterations = self.cores_height;

        // Assign each threads matrix component
        let mut a = submatrices_a.pop_front().unwrap();
        let mut b = submatrices_b.pop_front().unwrap();
        let mut c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut TaurusCoreInfo<Vec<Vec<T>>>| {
          for _ in 0..outer_iterations{
            c = F::matrix_mult(a, b, c, inner_iterations, core_info);
            a = F::inner_setup_a(c.clone(), core_info);
            b = F::inner_setup_b(c.clone(), core_info);
          }
          (i,j,c)
        };

        self.processor.run_core(core_function, core_info);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    self.processor.display_processor_time();
    matrix_c
  }
}

#[cfg(test)]
mod integration_tests;
