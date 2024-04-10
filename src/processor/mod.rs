use crate::broadcast::{BChannel, Sendable, Channel};
use std::{time::{Duration, Instant}, thread::{JoinHandle, self}};

struct CoreComm<T:Sendable>{
  left : Channel<(T, Duration)>,
  right : Channel<(T, Duration)>,
  up : Channel<(T, Duration)>,
  down : Channel<(T, Duration)>,
  row : BChannel<(T, Duration)>,
  col : BChannel<(T, Duration)>,
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
}

#[derive(Clone)]
struct CoreDebug {
  row : usize,
  col : usize,
  direct_count : usize,
  broadcast_count : usize,
  last_time : Instant,
  total_elapsed : Duration,
}

impl CoreDebug {
  fn new(row : usize, col : usize) -> CoreDebug {
    CoreDebug { 
      row,
      col,
      direct_count : 0,
      broadcast_count : 0,
      last_time: Instant::now(),
      total_elapsed: Duration::ZERO,
    }
  }

  fn get_curr_elapsed(&self) -> Duration {
    self.last_time.elapsed() + self.total_elapsed
  }

  fn update_elapsed(&mut self, outer : Duration) {
    let current = self.get_curr_elapsed();
    if current < outer {
      self.last_time = Instant::now();
      self.total_elapsed = outer;
    } else {
    }
  }

  fn set_elapsed(&mut self) {
    self.total_elapsed = self.get_curr_elapsed();
    self.last_time = Instant::now();
  }

  fn get_last_elapsed(&self) -> Duration {
    self.total_elapsed
  }
}

pub enum Taurus {
  LEFT,
  RIGHT,
  UP,
  DOWN,
  ROW,
  COL,
}

pub trait CoreInfo<T : Sendable> {
  type ChannelOption;

  fn send(&mut self, ch_option : Self::ChannelOption, data : T);
  fn recv(&mut self, ch_option : Self::ChannelOption) -> T;
}

pub struct TaurusCoreInfo<T : Sendable> {
  pub row : usize,
  pub col : usize,
  core_comm : CoreComm<T>,
  core_debug : CoreDebug,
} 

impl<T:Sendable> CoreInfo<T> for TaurusCoreInfo<T> {
  type ChannelOption = Taurus;

  fn send(&mut self, ch_option : Self::ChannelOption, data : T){
    let elapsed =  self.core_debug.get_curr_elapsed();
    match ch_option {
      Taurus::LEFT => {
        self.core_comm.left.send((data,elapsed));
        self.core_debug.direct_count += 1;
      },
      Taurus::RIGHT => {
        self.core_comm.right.send((data,elapsed));
        self.core_debug.direct_count += 1;
      },
      Taurus::UP => {
        self.core_comm.up.send((data,elapsed));
        self.core_debug.direct_count += 1;
      },
      Taurus::DOWN => {
        self.core_comm.down.send((data,elapsed));
        self.core_debug.direct_count += 1;
      },
      Taurus::ROW => {
        self.core_comm.row.send((data,elapsed));
        self.core_debug.broadcast_count += 1;
      },
      Taurus::COL => {
        self.core_comm.col.send((data,elapsed));
        self.core_debug.broadcast_count += 1;
      },
    }
  }

  fn recv(&mut self, ch_option : Self::ChannelOption) -> T{
    let (data, recv_time) = match ch_option {
      Taurus::LEFT => self.core_comm.left.recv(),
      Taurus::RIGHT => self.core_comm.right.recv(),
      Taurus::UP => self.core_comm.up.recv(),
      Taurus::DOWN => self.core_comm.down.recv(),
      Taurus::ROW => self.core_comm.row.recv(),
      Taurus::COL => self.core_comm.col.recv(),
    };
    self.core_debug.update_elapsed(recv_time);
    data
  }
}

#[derive(Copy,Clone,Debug, PartialEq)]
pub struct SubmatrixDim {
  pub start_row : usize,
  pub start_col : usize,
  pub width : usize,
  pub height : usize,
}

pub struct Processor<H : Sendable + 'static, T : Sendable + 'static> {
  pub rows : usize,
  pub cols : usize,
  handles : Vec<JoinHandle<(H, TaurusCoreInfo<T>)>>,
  debugs : Vec<CoreDebug>
}

impl<H : Sendable + 'static, T : Sendable + 'static> Processor<H, T> {
  pub fn new(rows : usize, cols : usize) -> Processor<H, T>{
    Processor {rows , cols , handles : Vec::new(), debugs : Vec::new() }
  }

  pub fn create_taurus(&self) -> Vec<TaurusCoreInfo<T>> {
      let num_cores = self.cols * self.rows;
      let mut cores : Vec<TaurusCoreInfo<T>> = Vec::with_capacity(num_cores);
      for row in 0..self.rows {
        for col in 0..self.cols {
          cores.push(TaurusCoreInfo{ row, col, core_comm : CoreComm::new(), 
            core_debug : CoreDebug::new(row, col)
          })
        }
      }

    for i in 0..self.rows {
      let mut bchannels : Vec<BChannel<(T, Duration)>> = BChannel::new(self.cols);
      for step in 0..self.cols {
        let core_index = self.rows * i + step;
        cores[core_index].core_comm.row = bchannels.pop().unwrap();
      }
    }

    for i in 0..self.cols {
      let mut bchannels : Vec<BChannel<(T, Duration)>> = BChannel::new(self.rows);
      for step in 0..self.rows {
        let core_index = self.rows * step + i;
        cores[core_index].core_comm.col = bchannels.pop().unwrap();
      }
    }
    
    for i in 0..num_cores {
      let (up, down) = Channel::new();
      let up_index = i;
      let down_index = ( num_cores + i - self.cols ) % num_cores;

      cores[up_index].core_comm.up = up;
      cores[down_index].core_comm.down = down; 

      let (right, left) = Channel::new();
      let right_index = i;
      let left_index = i - ( i % self.cols ) + ( (i +  1) % self.cols );

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

  fn get_matrix_slices<K:Clone>(matrix : &Vec<Vec<K>>, dims : &Vec<SubmatrixDim>) -> Vec<Vec<Vec<K>>> {
    dims.iter().map(|&dim| 
      matrix.iter().skip(dim.start_row).take(dim.height)
         .map(|row| row.iter().skip(dim.start_col).take(dim.width).cloned().collect::<Vec<_>>())
         .collect::<Vec<_>>()
    ).collect::<Vec<_>>()
  }

  pub fn get_submatrices<K: Clone>(&self, matrix : &Vec<Vec<K>>) -> Vec<Vec<Vec<K>>> {
    let matrix_rows = matrix.len();
    let matrix_cols = matrix[0].len();

    let submatrices_dim = self.get_submatrices_dim(matrix_rows, matrix_cols);
    
    Self::get_matrix_slices(matrix, &submatrices_dim)
  }

  pub fn run_core<F> (&mut self, f: F, mut core_info : TaurusCoreInfo<T>) 
  where
      F: FnOnce(&mut TaurusCoreInfo<T>) -> H + Send + 'static,
  {
        let handle = thread::spawn(move || {
          let result = f(&mut core_info);
          core_info.core_debug.set_elapsed();
          (result, core_info)
        });
        self.handles.push(handle);
  }

  pub fn collect_results (&mut self) -> Vec<H> {
    let mut results = Vec::new();
    while !self.handles.is_empty() {
      let handle = self.handles.pop().unwrap();
      let (result, core) = handle.join().unwrap();
      self.debugs.push(core.core_debug);
      results.push(result);
    }
    results
  }

  pub fn display_processor_time (&self) {
    for debug in &self.debugs {
      println!("Core {} {} elapsed time: {}µs",
               debug.row, debug.col, debug.get_last_elapsed().as_micros());
    }
  }
}


#[cfg(test)]
mod tests;
