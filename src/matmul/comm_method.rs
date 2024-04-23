use std::collections::VecDeque;
use crate::processor::debug::CoreDebugger;
use crate::processor::{TaurusCoreInfo, CoreInfo, Taurus, get_submatrices};
use crate::broadcast::Sendable;
use crate::types::Matrix;
use super::{Multiplicable, serial_matmul};


pub trait CommMethod<T: Multiplicable + Sendable, CoreType : CoreInfo<Matrix<T>>> {
  fn outer_setup_a(rows : usize, cols : usize, matrix_a : &Matrix<T>) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_a))
  }
  fn outer_setup_b(rows : usize, cols : usize, matrix_b : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_b))
  }
  fn outer_setup_c(rows : usize, cols : usize, matrix_c : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    VecDeque::from(get_submatrices(rows, cols, matrix_c))
  }
  fn inner_setup_a(a : Matrix<T>, _ : &mut CoreType, _ : &mut Option<&mut CoreDebugger>) 
    -> Matrix<T> {
    a
  }
  fn inner_setup_b(b : Matrix<T>, _ : &mut CoreType, _ : &mut Option<&mut CoreDebugger>) 
    -> Matrix<T> {
    b
  }
  fn matrix_mult(_ : Matrix<T>, _ : Matrix<T>, 
                                   _ : Matrix<T>, _ : usize, _ : &mut CoreType,
                                   _ : &mut Option<&mut CoreDebugger>) -> Matrix<T>;
}

pub struct Hash;

impl<T>  CommMethod<T, TaurusCoreInfo<Matrix<T>>> for Hash 
  where T : Sendable + Multiplicable {

  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>, 
                                     debugger : &mut Option<&mut CoreDebugger>) -> Matrix<T> {
    for iter in 0..iterations {
      if core_info.col == iter {
        core_info.debug_send(Taurus::ROW, matrix_a.clone(), debugger);
      }
      if core_info.row == iter {
      core_info.debug_send(Taurus::COL, matrix_b.clone(), debugger);
      }
      let received_a = core_info.debug_recv(Taurus::ROW, debugger);
      let received_b = core_info.debug_recv(Taurus::COL, debugger);

      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
    }
    return matrix_c;
  }
}

pub struct FoxOtto;

impl<T>  CommMethod<T, TaurusCoreInfo<Matrix<T>>> for FoxOtto 
  where T : Sendable + Multiplicable {
  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>,
                                     debugger : &mut Option<&mut CoreDebugger>) -> Matrix<T> {
    let mut received_b = matrix_b;
    for iter in 0..iterations {
      if iter == (( iterations + core_info.col - core_info.row) % iterations ) {
        core_info.debug_send(Taurus::ROW, matrix_a.clone(), debugger);
      }
      let received_a = core_info.debug_recv(Taurus::ROW, debugger);
      
      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
      
      core_info.debug_send(Taurus::UP, received_b, debugger);
      received_b = core_info.debug_recv(Taurus::DOWN, debugger);
    }
    return matrix_c;
  }
}

pub struct PipeFoxOtto;

impl<T>  CommMethod<T, TaurusCoreInfo<Matrix<T>>> for PipeFoxOtto 
  where T : Sendable + Multiplicable {
  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>,
                                     debugger : &mut Option<&mut CoreDebugger>) -> Matrix<T> {
    let mut received_b = matrix_b;
    for iter in 0..iterations {
      core_info.debug_send(Taurus::UP, received_b, debugger);
      if iter == (( iterations + core_info.col - core_info.row + 1)  % iterations ) {
        core_info.debug_send(Taurus::ROW, matrix_a.clone(), debugger);
      }
      received_b = core_info.debug_recv(Taurus::DOWN, debugger);
      let received_a = core_info.debug_recv(Taurus::ROW, debugger);
      
      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
      
    }
    return matrix_c;
  }
}

pub struct Cannon;


impl<T>  CommMethod<T, TaurusCoreInfo<Matrix<T>>> for Cannon 
  where T : Sendable + Multiplicable {
  fn outer_setup_a(rows : usize, cols : usize, matrix_a : &Matrix<T>,) -> VecDeque<Matrix<T>> {
    let submatrices_a = VecDeque::from(get_submatrices(rows, cols, matrix_a));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| row * cols +((cols + col - row) % cols))
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<T>>>();
    submatrices_a.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    return result;
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

    return result;
  }

  fn inner_setup_a (a : Matrix<T>, core_info : &mut TaurusCoreInfo<Matrix<T>>,
                    debugger : &mut Option<&mut CoreDebugger>) 
      -> Matrix<T> {
    let mut temp = a;
    for _ in 0..core_info.row {
      core_info.debug_send(Taurus::LEFT, temp, debugger);
      temp = core_info.debug_recv(Taurus::RIGHT, debugger);
    }
    temp
  }

  fn inner_setup_b (b : Matrix<T>, core_info : &mut TaurusCoreInfo<Matrix<T>>,
                    debugger : &mut Option<&mut CoreDebugger>) 
      -> Matrix<T> {
    let mut temp = b;
    for _ in 0..core_info.col {
      core_info.debug_send(Taurus::UP, temp, debugger);
      temp = core_info.debug_recv(Taurus::DOWN, debugger);
    }
    temp
  }

  fn matrix_mult(matrix_a : Matrix<T>, matrix_b : Matrix<T>, 
                                     mut matrix_c : Matrix<T>, iterations : usize,
                                     core_info : &mut TaurusCoreInfo<Matrix<T>>,
                                     debugger : &mut Option<&mut CoreDebugger>) -> Matrix<T> {
    let mut received_a = matrix_a;
    let mut received_b = matrix_b;

    for _ in 0..iterations {
      matrix_c = serial_matmul(&received_a, &received_b, &matrix_c);
      
      core_info.debug_send(Taurus::LEFT, received_a, debugger);
      core_info.debug_send(Taurus::UP, received_b, debugger);
      received_a = core_info.debug_recv(Taurus::RIGHT, debugger);
      received_b = core_info.debug_recv(Taurus::DOWN, debugger);
    }
    return matrix_c;
  }
}
