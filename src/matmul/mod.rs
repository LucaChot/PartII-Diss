use std::collections::VecDeque;
use crate::processor::debug::CoreDebugger;
use crate::processor::{TaurusCoreInfo, Processor, get_submatrices_dim};
use crate::broadcast::Sendable;
use crate::types::Matrix;

pub mod comm_method;
use comm_method::CommMethod;

pub trait Multiplicable { 
  fn initial_c (a : &Matrix<Self>, b : &Matrix<Self>) -> Matrix<Self> where Self: Sized;
  fn neutral_matrix (rows : usize, cols : usize) -> Matrix<Self> where Self: Sized;
  fn singleton_matrix(a : Self, b : Self, c : Self) -> Self;
}

pub fn serial_matmul<T : Multiplicable + Clone>(matrix_a : &Matrix<T>,
                                                       matrix_b : &Matrix<T>,
                                                       matrix_c : &Matrix<T>)
-> Matrix<T>{
  let rows_a = matrix_a.len();
  let cols_b = matrix_b[0].len();
  let cols_a = matrix_a[0].len();

  (0..rows_a)
    .map(|i| 
      (0..cols_b)
        .map(|j| 
          (0..cols_a)
            .fold(matrix_c[i][j].clone(), |acc, k| T::singleton_matrix(matrix_a[i][k].clone(), matrix_b[k][j].clone(), acc))
        ).collect::<Vec<T>>()
    ).collect::<Matrix<T>>()
}

pub struct MatMul<'a,T> 
where T : Multiplicable + Sendable + 'static {
  processor : &'a mut Processor<(usize, usize, Matrix<T>),Matrix<T>, TaurusCoreInfo<Matrix<T>>>
}

impl<'a,T> MatMul<'a,T> 
where T : Multiplicable + Sendable + 'static {
  pub fn new(processor : &'a mut Processor<(usize, usize, Matrix<T>),Matrix<T>, TaurusCoreInfo<Matrix<T>>>) -> Self {
    MatMul {
      processor 
    }
  }

  fn collect_c(&self, core_results : &Vec<(usize, usize, Matrix<T>)>,
               matrix_c : &mut Matrix<T>) {
    let m_rows = matrix_c.len();
    let m_cols = matrix_c[0].len();
    let submatrices_dim = get_submatrices_dim(self.processor.rows, self.processor.cols, m_rows, m_cols);

    // Assign the final values to the W and P matrix
    for (i, j, c)  in core_results {
      let index = i * self.processor.cols + j;
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
    where F : CommMethod<T,TaurusCoreInfo<Matrix<T>>>  {
    let mut cores_info : VecDeque<TaurusCoreInfo<Matrix<T>>> = VecDeque::from(self.processor.build_network());

    let mut submatrices_a = F::outer_setup_a(self.processor.rows, self.processor.cols, &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows, self.processor.cols, &matrix_b);
    let mut matrix_c = T::initial_c(&matrix_a, &matrix_b);
    let mut submatrices_c = F::outer_setup_c(self.processor.rows, self.processor.cols, &matrix_c);

    for i in 0..self.processor.rows {
      for j in 0..self.processor.cols {
        // Assign each thread its corresponding channels
        let core_info = cores_info.pop_front().unwrap();
        // Sender for returning the results
        let iterations = self.processor.rows;

        // Assign each threads matrix component
        let a = submatrices_a.pop_front().unwrap();
        let b = submatrices_b.pop_front().unwrap();
        let c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut TaurusCoreInfo<Vec<Vec<T>>>, debugger : &mut CoreDebugger| {
          let c = F::matrix_mult(a, b, c, iterations, core_info, &mut Some(debugger));
          (i,j,c)
        };

        self.processor.run_debug_core(core_function, core_info);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    matrix_c
  }   

  pub fn parallel_square<F> (&mut self, matrix_a : Matrix<T>, outer_iterations : usize)
    -> Matrix<T> 
    where F : CommMethod<T,TaurusCoreInfo<Matrix<T>>> {
    let mut cores_info : VecDeque<TaurusCoreInfo<Matrix<T>>> = VecDeque::from(self.processor.build_network());

    let mut submatrices_a = F::outer_setup_a(self.processor.rows, self.processor.cols, &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows, self.processor.cols, &matrix_a);
    let mut matrix_c = T::initial_c(&matrix_a, &matrix_a);
    let mut submatrices_c = F::outer_setup_c(self.processor.rows, self.processor.cols, &matrix_c);

    for i in 0..self.processor.rows {
      for j in 0..self.processor.cols {
        // Assign each thread its corresponding channels
        let core_info = cores_info.pop_front().unwrap();
        // Sender for returning the results
        let inner_iterations = self.processor.rows;

        // Assign each threads matrix component
        let mut a = submatrices_a.pop_front().unwrap();
        let mut b = submatrices_b.pop_front().unwrap();
        let mut c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut TaurusCoreInfo<Vec<Vec<T>>>, debugger : &mut CoreDebugger| {
          let mut debug = Some(debugger);
          for _ in 0..outer_iterations{
            c = F::matrix_mult(a, b, c, inner_iterations, core_info, &mut debug);
            a = F::inner_setup_a(c.clone(), core_info, &mut debug);
            b = F::inner_setup_b(c.clone(), core_info, &mut debug);
          }
          (i,j,c)
        };

        self.processor.run_debug_core(core_function, core_info);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    matrix_c
  }
}

    
#[cfg(test)]
mod tests;
