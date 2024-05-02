use std::collections::VecDeque;
use crate::processor::{taurus::TaurusOption, Core, get_submatrices};
use crate::broadcast::Sendable;
use crate::types::Matrix;
use super::{Multiplicable, serial_matmul};


pub trait CommMethod<T: Multiplicable + Sendable, CoreType : Core<Matrix<T>>> {
  fn outer_setup_a(rows : usize, cols : usize, matrix_a : &Matrix<T>) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_a))
  }
  fn outer_setup_b(rows : usize, cols : usize, matrix_b : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_b))
  }
  fn outer_setup_c(rows : usize, cols : usize, matrix_c : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_c))
  }
  fn inner_setup_a(a : Matrix<T>, _ : &mut CoreType) 
    -> Matrix<T> {
    a
  }
  fn inner_setup_b(b : Matrix<T>, _ : &mut CoreType) 
    -> Matrix<T> {
    b
  }
  fn matrix_mult(_ : Matrix<T>, _ : Matrix<T>, 
                                   _ : Matrix<T>, _ : usize, _ : &mut CoreType) -> Matrix<T>;
}

pub struct Hash;

impl<T, CoreType>  CommMethod<T, CoreType> for Hash 
  where T : Sendable + Multiplicable,
        CoreType : Core<Matrix<T>, ChannelOption = TaurusOption> {

  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut CoreType) -> Matrix<T> {
    for iter in 0..iterations {
      if core_info.col() == iter {
        core_info.send(matrix_a.clone(), &TaurusOption::ROW);
      }
      if core_info.row() == iter {
      core_info.send(matrix_b.clone(), &TaurusOption::COL);
      }
      let received_a = core_info.recv(&TaurusOption::ROW);
      let received_b = core_info.recv(&TaurusOption::COL);

      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
    }
    return matrix_c;
  }
}

pub struct FoxOtto;

impl<T, CoreType>  CommMethod<T, CoreType> for FoxOtto 
  where T : Sendable + Multiplicable,
        CoreType : Core<Matrix<T>, ChannelOption = TaurusOption> {
  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut CoreType) -> Matrix<T> {
    let mut received_b = matrix_b;
    for iter in 0..iterations {
      if iter == (( iterations + core_info.col() - core_info.row()) % iterations ) {
        core_info.send(matrix_a.clone(), &TaurusOption::ROW);
      }
      let received_a = core_info.recv(&TaurusOption::ROW);
      
      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
      
      core_info.send(received_b, &TaurusOption::UP);
      received_b = core_info.recv(&TaurusOption::DOWN);
    }
    return matrix_c;
  }
}

pub struct PipeFoxOtto;

impl<T, CoreType>  CommMethod<T, CoreType> for PipeFoxOtto 
  where T : Sendable + Multiplicable,
        CoreType : Core<Matrix<T>, ChannelOption = TaurusOption> {
  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut CoreType) -> Matrix<T> {
    let mut received_b = matrix_b;
    for iter in 0..iterations {
      core_info.send(received_b, &TaurusOption::UP);
      if iter == (( iterations + core_info.col() - core_info.row() - 1)  % iterations ) {
        core_info.send(matrix_a.clone(), &TaurusOption::ROW);
      }
      received_b = core_info.recv(&TaurusOption::DOWN);
      let received_a = core_info.recv(&TaurusOption::ROW);

      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
    }
    return matrix_c;
  }
}

pub struct Cannon;

impl<T, CoreType>  CommMethod<T, CoreType> for Cannon 
  where T : Sendable + Multiplicable,
        CoreType : Core<Matrix<T>, ChannelOption = TaurusOption> {
  fn outer_setup_a(rows : usize, cols : usize, matrix_a : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    let submatrices_a = VecDeque::from(get_submatrices(rows, cols, matrix_a));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| row * cols +((cols + col - row) % cols))
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<T>>>();
    submatrices_a.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    result
  }

  fn outer_setup_b(rows : usize, cols : usize, matrix_b : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    let submatrices_b = VecDeque::from(get_submatrices(rows, cols, matrix_b));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| ((rows + row - col) % rows) * cols + col)
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<T>>>();
    submatrices_b.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    result
  }

  fn inner_setup_a (a : Matrix<T>, core_info : &mut CoreType) 
      -> Matrix<T> {
    let mut temp = a;
    for _ in 0..core_info.row() {
      core_info.send(temp, &TaurusOption::LEFT);
      temp = core_info.recv(&TaurusOption::RIGHT);
    }
    temp
  }

  fn inner_setup_b (b : Matrix<T>, core_info : &mut CoreType) 
      -> Matrix<T> {
    let mut temp = b;
    for _ in 0..core_info.col() {
      core_info.send(temp, &TaurusOption::UP);
      temp = core_info.recv(&TaurusOption::DOWN);
    }
    temp
  }

  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut CoreType) -> Matrix<T> {
    let mut received_a = matrix_a;
    let mut received_b = matrix_b;

    for _ in 0..iterations {
      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
      
      core_info.send(received_a, &TaurusOption::LEFT);
      core_info.send(received_b, &TaurusOption::UP);
      received_a = core_info.recv(&TaurusOption::RIGHT);
      received_b = core_info.recv(&TaurusOption::DOWN);
    }
    return matrix_c;
  }
}
