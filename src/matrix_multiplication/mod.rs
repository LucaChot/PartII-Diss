use std::{fmt::{Debug,Display,Formatter,Result}, collections::VecDeque};
use crate::processor::CoreInfo;
use crate::broadcast::Sendable;
use crate::processor::{get_submatrices, get_submatrices_dim};

#[derive(Clone,Debug)]
pub struct Msg {
  w : isize,
  p : usize,
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.w, self.p)
    }
}

impl Msg {
  pub fn new(w : isize, p : usize) -> Msg {
    return Msg {
      w,
      p,
    }
  }

  pub fn get_w(&self) -> isize {
    return self.w;
  }

  pub fn get_p(&self) -> usize {
    return self.p;
  }
}

impl Sendable for Msg {}

impl<T:Sendable> Sendable for Vec<Vec<T>> {}
pub type Matrix<T> = Vec<Vec<T>>;

pub fn singleton_matrix_multiplication(a : isize, b : isize, c : isize) -> isize {
  c + a * b
}

pub fn singleton_pred_matrix_multiplication(a : Msg, b : Msg, mut c : Msg) -> Msg {
  if a.w != -1 && b.w != -1 && ( c.w == -1 || a.w + b.w < c.w ){
    c.w = a.w + b.w;
    c.p = b.p;
  }
  c
}

pub fn serial_matrix_multiplication<T : Sendable>(matrix_a : &Matrix<T>, matrix_b : &Matrix<T>, matrix_c : &Matrix<T>, 
                                               f : fn(T,T,T)->T)
-> Matrix<T>{
  let rows_a = matrix_a.len();
  let cols_b = matrix_b[0].len();
  let cols_a = matrix_a[0].len();

  (0..rows_a)
    .map(|i| 
      (0..cols_b)
        .map(|j| 
          (0..cols_a)
            .fold(matrix_c[i][j].clone(), |acc, k| f(matrix_a[i][k].clone(), matrix_b[k][j].clone(), acc))
        ).collect::<Vec<T>>()
    ).collect::<Matrix<T>>()
}

//TODO : Implement default
pub trait ParallelMatMult {
  fn setup_a<T : Sendable>(matrix_a : &Matrix<T>,
                                 (rows, cols) : (usize, usize)) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(matrix_a, (rows, cols)))
  }
  fn setup_b<T : Sendable>(matrix_b : &Matrix<T>,
                                 (rows, cols) : (usize, usize)) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(matrix_b, (rows, cols)))
  }
  fn setup_c<T : Sendable>(matrix_c : &Matrix<T>,
                                 (rows, cols) : (usize, usize)) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(matrix_c, (rows, cols)))
  }
  fn matrix_mult<T : Sendable>(_ : Matrix<T>, _ : Matrix<T>, 
                                   _ : Matrix<T>, _ : usize,
                                   _ : &CoreInfo<Matrix<T>>, 
                                   _ : fn(T,T,T)->T) -> Matrix<T>;
}

pub struct Hash { } 

impl ParallelMatMult for Hash {
  fn matrix_mult<T : Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &CoreInfo<Matrix<T>>, 
                                     func : fn(T,T,T)->T) -> Matrix<T> {
    for iter in 0..iterations {
      if core_info.col == iter {
        core_info.core_comm.row.send(matrix_a.clone());
      }
      if core_info.row == iter {
        core_info.core_comm.col.send(matrix_b.clone());
      }
      let received_a = core_info.core_comm.row.recv().unwrap();
      let received_b = core_info.core_comm.col.recv().unwrap();

      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c, func);
    }
    return matrix_c;
  }
}

pub struct FoxOtto { } 

impl ParallelMatMult for FoxOtto {
  fn matrix_mult<T : Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &CoreInfo<Matrix<T>>, 
                                     func : fn(T,T,T)->T) -> Matrix<T> {
    let mut received_b = matrix_b;

    for iter in 0..iterations {
      if iter == (( iterations + core_info.col - core_info.row) % iterations ) {
        core_info.core_comm.row.send(matrix_a.clone());
      }
      let received_a = core_info.core_comm.row.recv().unwrap();
      
      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c, func);
      
      let _ = core_info.core_comm.up.send(received_b);
      received_b = core_info.core_comm.down.recv().unwrap();
    }
    return matrix_c;
  }
}

pub struct Cannon { } 

impl ParallelMatMult for Cannon {
  fn setup_a<T : Sendable>(matrix_a : &Matrix<T>,
                                 (rows, cols) : (usize, usize)) -> VecDeque<Matrix<T>> {
    let submatrices_a = VecDeque::from(get_submatrices(matrix_a, (rows, cols)));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| row * cols +((cols + col - row) % cols))
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<T>>>();
    submatrices_a.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    return result;
  }

  fn setup_b<T : Sendable>(matrix_b : &Matrix<T>,
                                 (rows, cols) : (usize, usize)) -> VecDeque<Matrix<T>> {
    let submatrices_b = VecDeque::from(get_submatrices(matrix_b, (rows, cols)));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| ((rows + row - col) % rows) * cols + col)
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<T>>>();
    submatrices_b.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    return result;
  }

  fn matrix_mult<T : Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &CoreInfo<Matrix<T>>, 
                                     func : fn(T,T,T)->T) -> Matrix<T> {
    let mut received_a = matrix_a;
    let mut received_b = matrix_b;

    for _ in 0..iterations {
      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c, func);
      
      core_info.core_comm.left.send(received_a);
      core_info.core_comm.up.send(received_b);
      received_a = core_info.core_comm.right.recv().unwrap();
      received_b = core_info.core_comm.down.recv().unwrap();
    }
    return matrix_c;
  }
}

    
#[cfg(test)]
mod tests;
