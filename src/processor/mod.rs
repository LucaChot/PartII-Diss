use crate::broadcast::{BChannel, Sendable, Channel};
use std::{time::{Duration, Instant}, cell::RefCell, sync::{Arc, Mutex}, thread::JoinHandle};

pub struct CoreComm<T:Sendable>{
  pub left : Channel<T>,
  pub right : Channel<T>,
  pub up : Channel<T>,
  pub down : Channel<T>,
  pub row : BChannel<T>,
  pub col : BChannel<T>,
}

impl<T : Sendable> CoreComm<T> {
  fn new() -> CoreComm<T> {
    CoreComm { 
      left: Channel::empty(),
      right: Channel::empty(),
      up: Channel::empty(),
      down: Channel::empty(),
      row: BChannel::empty(),
      col: BChannel::empty()
    }
  } 

  pub fn num_broadcasts(&self) -> usize {
    self.row.get_sent() + self.col.get_sent()
  }

  pub fn num_direct(&self) -> usize {
    self.left.get_sent() + self.right.get_sent() + self.up.get_sent() + self.down.get_sent()
  }
}

#[derive(Clone)]
pub struct CoreDebug {
  last_time : Instant,
  total_elapsed : Duration,
}

impl CoreDebug {
  pub fn new() -> CoreDebug {
    CoreDebug { 
      last_time: Instant::now(),
      total_elapsed: Duration::ZERO,
    }

  }
  pub fn get_elapsed(&self) -> Duration {
    self.last_time.elapsed() + self.total_elapsed
  }

  pub fn update_elapsed(&mut self, outer : Duration) {
    let current = self.get_elapsed();
    if current < outer {
      self.last_time = Instant::now();
      self.total_elapsed = outer;
    }
  }
}

pub struct CoreInfo<T : Sendable> {
  pub row : usize,
  pub col : usize,
  pub core_comm : CoreComm<T>,
  core_debug : Arc<Mutex<RefCell<CoreDebug>>>,
} 

#[derive(Copy,Clone,Debug, PartialEq)]
pub struct SubmatrixDim {
  pub start_row : usize,
  pub start_col : usize,
  pub width : usize,
  pub height : usize,
}

pub struct Processor<T> {
  pub rows : usize,
  pub cols : usize,
  pub handles : Vec<JoinHandle<T>>
}

impl<H> Processor<H> {
  pub fn new(rows : usize, cols : usize) -> Processor<H>{
    Processor {rows , cols , handles : Vec::new() }
  }

  pub fn create_taurus<T: Sendable>(&self) -> Vec<CoreInfo<T>> {
      let num_cores = self.cols * self.rows;
      let mut cores : Vec<CoreInfo<T>> = Vec::with_capacity(num_cores);
      for row in 0..self.rows {
        for col in 0..self.cols {
          cores.push(CoreInfo{ row, col, core_comm : CoreComm::new(), 
            core_debug : Arc::new(Mutex::new(RefCell::new(CoreDebug::new())))
          })
        }
      }

    for i in 0..self.rows {
      let mut bchannels : Vec<BChannel<T>> = BChannel::new(self.cols);
      for step in 0..self.cols {
        let core_index = self.rows * i + step;
        let mut bchannel = bchannels.pop().unwrap();
        bchannel.set_core_debug(Arc::clone(&cores[core_index].core_debug));
        cores[core_index].core_comm.row = bchannel;
      }
    }

    for i in 0..self.cols {
      let mut bchannels : Vec<BChannel<T>> = BChannel::new(self.rows);
      for step in 0..self.rows {
        let core_index = self.rows * step + i;
        let mut bchannel = bchannels.pop().unwrap();
        bchannel.set_core_debug(Arc::clone(&cores[core_index].core_debug));
        cores[core_index].core_comm.col = bchannel;
      }
    }
    
    for i in 0..num_cores {
      let (mut up, mut down) = Channel::new();
      let up_index = i;
      let down_index = ( num_cores + i - self.cols ) % num_cores;

      up.set_core_debug(Arc::clone(&cores[up_index].core_debug));
      down.set_core_debug(Arc::clone(&cores[down_index].core_debug));

      cores[up_index].core_comm.up = up;
      cores[down_index].core_comm.down = down; 

      let (mut right, mut left) = Channel::new();
      let right_index = i;
      let left_index = i - ( i % self.cols ) + ( (i +  1) % self.cols );

      right.set_core_debug(Arc::clone(&cores[right_index].core_debug));
      left.set_core_debug(Arc::clone(&cores[left_index].core_debug));

      cores[right_index].core_comm.right = right;
      cores[left_index].core_comm.left = left; 
    }
    
    return cores
  }

  /// This function returns a Vec containing the dimensions of the submatrices to 
  /// be assigned to each processor given the length of the array of processors 
  /// and the matrix along a given axis
  ///
  /// # Arguemnts
  /// * `processor_length` - Length of processor along a given axis
  /// * `matrix_length` - Length of matrix along a given axis
  ///
  /// # Returns
  /// Returns the Vec<usize> of length `processor_length` which contains the 
  /// length along the axis of the submatrix to be assigned to each processor
  fn get_submatrices_dim_along_axis(processor_length : usize, matrix_length : usize) -> Vec<usize> {
    let min_len : usize = matrix_length / processor_length;
    let remaining : usize = matrix_length - ( processor_length * min_len );
    let mut submatrix_dimensions : Vec<usize> = vec![min_len; processor_length]; 

    for element in submatrix_dimensions[0..remaining].iter_mut() {
      *element += 1;
    }

    submatrix_dimensions
  }

  pub fn get_submatrices_dim(&self, matrix_rows : usize, matrix_cols : usize) -> Vec<SubmatrixDim> {
    let dim_along_y = Self::get_submatrices_dim_along_axis(self.rows, matrix_rows);
    let dim_along_x = Self::get_submatrices_dim_along_axis(self.cols, matrix_cols);

    dim_along_y.iter().fold((0, Vec::new()), |(start_row, mut result), &height| {
      dim_along_x.iter().fold(0, |start_col, &width| {
        result.push(SubmatrixDim {
          start_row,
          start_col,
          width,
          height,
        });
        start_col + width
      });
      (start_row + height, result)
      }).1
  }

  fn get_matrix_slices<T:Clone>(matrix : &Vec<Vec<T>>, dims : &Vec<SubmatrixDim>) -> Vec<Vec<Vec<T>>> {
    dims.iter().map(|&dim| 
      matrix.iter().skip(dim.start_row).take(dim.height)
         .map(|row| row.iter().skip(dim.start_col).take(dim.width).cloned().collect::<Vec<_>>())
         .collect::<Vec<_>>()
    ).collect::<Vec<_>>()
  }

  pub fn get_submatrices<T: Clone>(&self, matrix : &Vec<Vec<T>>) -> Vec<Vec<Vec<T>>> {
    let matrix_rows = matrix.len();
    let matrix_cols = matrix[0].len();

    let submatrices_dim = self.get_submatrices_dim(matrix_rows, matrix_cols);
    
    Self::get_matrix_slices(matrix, &submatrices_dim)
  }
}


#[cfg(test)]
mod tests;
