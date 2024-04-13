use std::collections::VecDeque;
use crate::processor::{TaurusCoreInfo, CoreInfo, Taurus, get_submatrices};
use crate::broadcast::Sendable;
use crate::types::Matrix;

pub trait Multiplicable { 
  fn neutral_element (rows : usize, cols : usize) -> Matrix<Self> where Self: Sized;
  fn singleton_matrix<T : Multiplicable>(a : Self, b : Self, c : Self) -> Self;
}

fn serial_matrix_multiplication<T : Multiplicable + Clone>(matrix_a : &Matrix<T>,
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
            .fold(matrix_c[i][j].clone(), |acc, k| T::singleton_matrix::<T>(matrix_a[i][k].clone(), matrix_b[k][j].clone(), acc))
        ).collect::<Vec<T>>()
    ).collect::<Matrix<T>>()
}

//TODO : Implement default
pub trait ParallelMatMult {
  fn outer_setup_a<K : Clone>(rows : usize, cols : usize, matrix_a : &Matrix<K>) -> VecDeque<Matrix<K>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_a))
  }
  fn outer_setup_b<K : Clone>(rows : usize, cols : usize, matrix_b : &Matrix<K>,) -> VecDeque<Matrix<K>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_b))
  }
  fn outer_setup_c<K : Clone>(rows : usize, cols : usize, matrix_c : &Matrix<K>,) -> VecDeque<Matrix<K>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_c))
  }
  fn inner_setup_a<T:Sendable>(a : Matrix<T>, _ : &mut TaurusCoreInfo<Matrix<T>>) 
    -> Matrix<T> {
    a
  }
  fn inner_setup_b<T:Sendable>(b : Matrix<T>, _ : &mut TaurusCoreInfo<Matrix<T>>) 
    -> Matrix<T> {
    b
  }
  fn matrix_mult<T : Multiplicable + Sendable>(_ : Matrix<T>, _ : Matrix<T>, 
                                   _ : Matrix<T>, _ : usize, _ : &mut TaurusCoreInfo<Matrix<T>>,
                                   ) -> Matrix<T>;
}

pub struct Hash;

impl ParallelMatMult for Hash {

  fn matrix_mult<T : Sendable + Multiplicable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>, 
                                     ) -> Matrix<T> {
    for iter in 0..iterations {
      if core_info.col == iter {
        core_info.send(Taurus::ROW, matrix_a.clone());
      }
      if core_info.row == iter {
      core_info.send(Taurus::COL, matrix_b.clone());
      }
      let received_a = core_info.recv(Taurus::ROW);
      let received_b = core_info.recv(Taurus::COL);

      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c);
    }
    return matrix_c;
  }
}

pub struct FoxOtto;

impl ParallelMatMult for FoxOtto {
  fn matrix_mult<T : Multiplicable + Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>
                                     ) -> Matrix<T> {
    let mut received_b = matrix_b;
    for iter in 0..iterations {
      if iter == (( iterations + core_info.col - core_info.row) % iterations ) {
        core_info.send(Taurus::ROW, matrix_a.clone());
      }
      let received_a = core_info.recv(Taurus::ROW);
      
      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c);
      
      core_info.send(Taurus::UP, received_b);
      received_b = core_info.recv(Taurus::DOWN);
    }
    return matrix_c;
  }
}

pub struct Cannon;


impl ParallelMatMult for Cannon {
  fn outer_setup_a<K : Clone>(rows : usize, cols : usize, matrix_a : &Matrix<K>,) -> VecDeque<Matrix<K>> {
    let submatrices_a = VecDeque::from(get_submatrices(rows, cols, matrix_a));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| row * cols +((cols + col - row) % cols))
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<K>>>();
    submatrices_a.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    return result;
  }

  fn outer_setup_b<K : Clone>(rows : usize, cols : usize, matrix_b : &Matrix<K>,) -> VecDeque<Matrix<K>> {
    let submatrices_b = VecDeque::from(get_submatrices(rows, cols, matrix_b));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| ((rows + row - col) % rows) * cols + col)
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<K>>>();
    submatrices_b.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    return result;
  }

  fn inner_setup_a <T : Sendable>(a : Matrix<T>, core_info : &mut TaurusCoreInfo<Matrix<T>>) 
      -> Matrix<T> {
    let mut temp = a;
    for _ in 0..core_info.row {
      core_info.send(Taurus::LEFT, temp);
      temp = core_info.recv(Taurus::RIGHT);
    }
    temp
  }

  fn inner_setup_b<T : Sendable>(b : Matrix<T>, core_info : &mut TaurusCoreInfo<Matrix<T>>) 
      -> Matrix<T> {
    let mut temp = b;
    for _ in 0..core_info.col {
      core_info.send(Taurus::UP, temp);
      temp = core_info.recv(Taurus::DOWN);
    }
    temp
  }

  fn matrix_mult<T : Multiplicable + Sendable>(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>
                                     ) -> Matrix<T> {
    let mut received_a = matrix_a;
    let mut received_b = matrix_b;

    for _ in 0..iterations {
      matrix_c = serial_matrix_multiplication(&received_a, &received_b, &matrix_c);
      
      core_info.send(Taurus::LEFT, received_a);
      core_info.send(Taurus::UP, received_b);
      received_a = core_info.recv(Taurus::RIGHT);
      received_b = core_info.recv(Taurus::DOWN);
    }
    return matrix_c;
  }
}

    
#[cfg(test)]
mod tests;
