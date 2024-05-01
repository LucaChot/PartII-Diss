use crate::processor::{ProbeProcessor, Core};
use crate::processor::probe::Prober;
use crate::processor::{taurus::TaurusCore, get_submatrices_dim, Processor};
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
  processor : &'a mut Processor<(usize, usize, Matrix<T>),Matrix<T>, TaurusCore<Matrix<T>>>
}

impl<'a,T> MatMul<'a,T> 
where T : Multiplicable + Sendable + 'static {
  pub fn new(processor : &'a mut Processor<(usize, usize, Matrix<T>),Matrix<T>, TaurusCore<Matrix<T>>>) -> Self {
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
    where F : CommMethod<T,TaurusCore<Matrix<T>>>  {

    let rows = self.processor.rows;
    let cols = self.processor.cols;
    let mut submatrices_a = F::outer_setup_a(rows, cols, &matrix_a);
    let mut submatrices_b = F::outer_setup_b(rows, cols, &matrix_b);
    let mut matrix_c = T::initial_c(&matrix_a, &matrix_b);
    let mut submatrices_c = F::outer_setup_c(rows, cols, &matrix_c);

    for i in 0..rows {
      for j in 0..cols {
        // Sender for returning the results
        let iterations = rows;

        // Assign each threads matrix component
        let a = submatrices_a.pop_front().unwrap();
        let b = submatrices_b.pop_front().unwrap();
        let c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut TaurusCore<Vec<Vec<T>>>| {
          let c = F::matrix_mult(a, b, c, iterations, core_info);
          (i,j,c)
        };

        self.processor.run_core(core_function);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    matrix_c
  }   

  pub fn parallel_square<F> (&mut self, matrix_a : Matrix<T>, outer_iterations : usize)
    -> Matrix<T> 
    where F : CommMethod<T,TaurusCore<Matrix<T>>> {

    let mut submatrices_a = F::outer_setup_a(self.processor.rows, self.processor.cols, &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows, self.processor.cols, &matrix_a);
    let mut matrix_c = T::initial_c(&matrix_a, &matrix_a);
    let mut submatrices_c = F::outer_setup_c(self.processor.rows, self.processor.cols, &matrix_c);

    for i in 0..self.processor.rows {
      for j in 0..self.processor.cols {
        // Sender for returning the results
        let inner_iterations = self.processor.rows;

        // Assign each threads matrix component
        let mut a = submatrices_a.pop_front().unwrap();
        let mut b = submatrices_b.pop_front().unwrap();
        let mut c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut TaurusCore<Vec<Vec<T>>>| {
          for _ in 0..outer_iterations{
            c = F::matrix_mult(a, b, c, inner_iterations, core_info);
            a = F::inner_setup_a(c.clone(), core_info);
            b = F::inner_setup_b(c.clone(), core_info);
          }
          (i,j,c)
        };

        self.processor.run_core(core_function);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    matrix_c
  }
}

pub struct ProbeMatMul<'a, T, D, U, CoreType> 
where T : Multiplicable + Sendable + 'static,
      D : Sendable + 'static,
      U : Sendable + 'static,
      CoreType : Core<U> + Send,
      {
  processor : &'a mut ProbeProcessor<D, (usize, usize, Matrix<T>), U, CoreType>
}

impl<'a, T, D, U, CoreType> ProbeMatMul<'a, T, D, U, CoreType> 
where T : Multiplicable + Sendable + 'static,
      D : Sendable + 'static,
      U : Sendable + 'static,
      CoreType : Core<U> + Send + 'static,
      {
  pub fn new(processor : &'a mut ProbeProcessor<D,(usize, usize, Matrix<T>),U, CoreType>) -> Self {
    ProbeMatMul {
      processor 
    }
  }

  fn collect_c(&self, core_results : &Vec<(usize, usize, Matrix<T>)>,
               matrix_c : &mut Matrix<T>) {
    let m_rows = matrix_c.len();
    let m_cols = matrix_c[0].len();
    let submatrices_dim = get_submatrices_dim(self.processor.rows(), self.processor.cols(), m_rows, m_cols);

    // Assign the final values to the W and P matrix
    for (i, j, c)  in core_results {
      let index = i * self.processor.cols() + j;
      let submatrix_dim = submatrices_dim[index];
      for i in 0..submatrix_dim.height {
        for j in 0..submatrix_dim.width {
          matrix_c[submatrix_dim.start_row+i][submatrix_dim.start_col+j] = c[i][j].clone();
        }
      }
    }
  }
  
  pub fn parallel_mult<F,P>  (&mut self, matrix_a : Matrix<T>, matrix_b : Matrix<T>)
    -> Matrix<T> 
    where F : CommMethod<T,P>,
          P : Prober<D, U, CoreType> + Core<Matrix<T>>{

    let mut submatrices_a = F::outer_setup_a(self.processor.rows(), self.processor.cols(), &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows(), self.processor.cols(), &matrix_b);
    let mut matrix_c = T::initial_c(&matrix_a, &matrix_b);
    let mut submatrices_c = F::outer_setup_c(self.processor.rows(), self.processor.cols(), &matrix_c);

    for i in 0..self.processor.rows() {
      for j in 0..self.processor.cols() {
        // Sender for returning the results
        let iterations = self.processor.rows();

        // Assign each threads matrix component
        let a = submatrices_a.pop_front().unwrap();
        let b = submatrices_b.pop_front().unwrap();
        let c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut P| {
          let c = F::matrix_mult(a, b, c, iterations, core_info);
          (i,j,c)
        };

        self.processor.run_core(core_function);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    matrix_c
  }   

  pub fn parallel_square<F,P> (&mut self, matrix_a : Matrix<T>, outer_iterations : usize)
    -> Matrix<T> 
    where F : CommMethod<T,P>,
          P : Prober<D, U, CoreType> + Core<Matrix<T>>{

    let mut submatrices_a = F::outer_setup_a(self.processor.rows(), self.processor.cols(), &matrix_a);
    let mut submatrices_b = F::outer_setup_b(self.processor.rows(), self.processor.cols(), &matrix_a);
    let mut matrix_c = T::initial_c(&matrix_a, &matrix_a);
    let mut submatrices_c = F::outer_setup_c(self.processor.rows(), self.processor.cols(), &matrix_c);

    for i in 0..self.processor.rows() {
      for j in 0..self.processor.cols() {
        // Sender for returning the results
        let inner_iterations = self.processor.rows();

        // Assign each threads matrix component
        let mut a = submatrices_a.pop_front().unwrap();
        let mut b = submatrices_b.pop_front().unwrap();
        let mut c = submatrices_c.pop_front().unwrap();

        let core_function = move |core_info: &mut P| {
          for _ in 0..outer_iterations{
            c = F::matrix_mult(a, b, c, inner_iterations, core_info);
            a = F::inner_setup_a(c.clone(), core_info);
            b = F::inner_setup_b(c.clone(), core_info);
          }
          (i,j,c)
        };

        self.processor.run_core(core_function);
      }
    }

    let core_results = self.processor.collect_results();
    self.collect_c(&core_results, &mut matrix_c);
    matrix_c
  }
}

    
#[cfg(test)]
mod tests;
