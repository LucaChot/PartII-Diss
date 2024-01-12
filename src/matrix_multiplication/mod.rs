use std::fmt::{Debug,Display,Formatter,Result};
use crate::broadcast::Sendable;

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

pub fn serial_matrix_multiplication<T : Clone + Debug>(matrix_a : &Matrix<T>, matrix_b : &Matrix<T>, matrix_c : &Matrix<T>, 
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

pub mod hash {
  use crate::broadcast::{BChannel, Sendable};
  use super::serial_matrix_multiplication;
  pub use super::{Msg, Matrix, singleton_matrix_multiplication, singleton_pred_matrix_multiplication};

  pub struct ProcessorInfo<T : Sendable> {
    row : usize,
    col : usize,
    row_broadcast : BChannel<T>,
    col_broadcast : BChannel<T>,
  }

  impl<T : Sendable> ProcessorInfo<T> {
    pub fn new(row : usize, col : usize, row_broadcast : BChannel<T>, col_broadcast : BChannel<T>)
      -> ProcessorInfo<T> {
      ProcessorInfo { row, col, row_broadcast, col_broadcast}
    }
  }

  pub fn thread_matrix_mult<T : Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, mut matrix_c : Matrix<T>,
                            iteration : usize, p_info : &ProcessorInfo<Matrix<T>>, func : fn(T,T,T)->T)
    -> Matrix<T> {
    for iter in 0..iteration {
      if p_info.col == iter {
        p_info.row_broadcast.send(matrix_a.clone());
      }
      if p_info.row == iter {
        p_info.col_broadcast.send(matrix_b.clone());
      }
      let received_a = p_info.row_broadcast.recv().unwrap();
      let received_b = p_info.col_broadcast.recv().unwrap();

      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c, func);
    }
    return matrix_c;
  }

}

pub mod fox_otto {
  use std::sync::mpsc;
  use crate::broadcast::{BChannel, Sendable};
  use super::serial_matrix_multiplication;
  pub use super::{Msg, Matrix, singleton_matrix_multiplication, singleton_pred_matrix_multiplication};

  pub struct FoxOttoProcessorInfo<T : Sendable> {
    row : usize,
    col : usize,
    row_broadcast : BChannel<T>,
    tx : mpsc::Sender<T>,
    rx : mpsc::Receiver<T>,
  }

  impl<T : Sendable> FoxOttoProcessorInfo<T> {
    pub fn new(row : usize, col : usize, row_broadcast : BChannel<T>, tx : mpsc::Sender<T>, rx : mpsc::Receiver<T>)
      -> FoxOttoProcessorInfo<T> {
      FoxOttoProcessorInfo { row, col, row_broadcast, tx, rx}
    }
  }

  pub fn fox_otto_matrix_mult<T : Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, mut matrix_c : Matrix<T>,
                                            iterations : usize, p_info : &FoxOttoProcessorInfo<Matrix<T>>, func : fn(T,T,T)->T) 
    -> Matrix<T> {
    let mut received_b = matrix_b;

    for iter in 0..iterations {
      if iter == (( iterations + p_info.col - p_info.row) % iterations ) {
        p_info.row_broadcast.send(matrix_a.clone());
      }
      let received_a = p_info.row_broadcast.recv().unwrap();
      
      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c, func);
      
      let _ = p_info.tx.send(received_b);
      received_b = p_info.rx.recv().unwrap();
    }
    return matrix_c;
  }

}
    
#[cfg(test)]
mod tests;
