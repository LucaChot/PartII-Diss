use crate::broadcast::{Broadcast, Sendable, Direct, Channel};
use std::{time::Duration, thread::{JoinHandle, self}, mem::size_of_val, ops::{Mul, Div}};

pub mod debug;

use debug::{CoreDebug, CoreDebugger};

struct CoreComm<T:Sendable>{
  left : Direct<(T, Option<Duration>)>,
  right : Direct<(T, Option<Duration>)>,
  up : Direct<(T, Option<Duration>)>,
  down : Direct<(T, Option<Duration>)>,
  row : Broadcast<(T, Option<Duration>)>,
  col : Broadcast<(T, Option<Duration>)>,
}

impl<T : Sendable> CoreComm<T> {
  fn new() -> CoreComm<T> {
    CoreComm { 
      left: Direct::empty(),
      right: Direct::empty(),
      up: Direct::empty(),
      down: Direct::empty(),
      row: Broadcast::empty(),
      col: Broadcast::empty()
    }
  } 
}

#[derive(Clone,Copy)]
struct CommInfo {
  latency : Duration,
  bandwidth  : usize,
  startup : Duration,
  broadcast_size : usize
}

impl CommInfo {
  fn direct_time<T>(&self, item : &T) -> Duration {
    Duration::new(size_of_val(item) as u64,0).div(self.bandwidth as u32)
  }

  fn broadcast_time<T>(&self, item : &T) -> Duration {
    self.startup.mul(self.broadcast_size as u32)
      + Duration::new(size_of_val(item) as u64,0).div(self.bandwidth as u32)
  }
}


pub trait CoreInfo<T : Sendable> : Send {
  type ChannelOption;

  fn get_row(&self) -> usize;
  fn get_col(&self) -> usize;
  fn debug_send(&mut self, ch_option : Self::ChannelOption, data : T, debugger : &mut Option<&mut CoreDebugger>);
  fn debug_recv(&mut self, ch_option : Self::ChannelOption, debugger : &mut Option<&mut CoreDebugger>) -> T;
  fn send(&mut self, ch_option : Self::ChannelOption, data : T);
  fn recv(&mut self, ch_option : Self::ChannelOption) -> T;
}

pub enum Taurus {
  LEFT,
  RIGHT,
  UP,
  DOWN,
  ROW,
  COL,
}

pub struct TaurusCoreInfo<T : Sendable> {
  pub row : usize,
  pub col : usize,
  core_comm : CoreComm<T>,
  comm_info : CommInfo
} 

impl<T:Sendable> CoreInfo<T> for TaurusCoreInfo<T> {
  type ChannelOption = Taurus;

  fn debug_send(&mut self, ch_option : Self::ChannelOption, data : T, debugger : &mut Option<&mut CoreDebugger>){
    let comm_cost  = match  ch_option {
      Taurus::ROW | Taurus::COL => self.comm_info.broadcast_time(&data),
      _ => self.comm_info.direct_time(&data)
    };
    let recv_time =  match debugger {
      Some(core_debugger) => {
        core_debugger.increment_time(comm_cost);
        Some(core_debugger.get_curr_elapsed() + self.comm_info.latency)
      },
      None => None,
    };
    match ch_option {
      Taurus::LEFT => self.core_comm.left.send((data,recv_time)),
      Taurus::RIGHT => self.core_comm.right.send((data,recv_time)),
      Taurus::UP => self.core_comm.up.send((data,recv_time)),
      Taurus::DOWN => self.core_comm.down.send((data,recv_time)),
      Taurus::ROW => self.core_comm.row.send((data,recv_time)),
      Taurus::COL => self.core_comm.col.send((data,recv_time)),
    }
   match debugger {
      Some(core_debugger) => {
        match  ch_option {
          Taurus::ROW | Taurus::COL => core_debugger.increment_broadcast(),
          _ => core_debugger.increment_direct()
        }
      }
      None => ()
   }
  }


  fn debug_recv(&mut self, ch_option : Self::ChannelOption, debugger : &mut Option<&mut CoreDebugger>) -> T{
    let (data, recv_time) = match ch_option {
      Taurus::LEFT => self.core_comm.left.recv(),
      Taurus::RIGHT => self.core_comm.right.recv(),
      Taurus::UP => self.core_comm.up.recv(),
      Taurus::DOWN => self.core_comm.down.recv(),
      Taurus::ROW => self.core_comm.row.recv(),
      Taurus::COL => self.core_comm.col.recv(),
    };
    match debugger {
      Some(core_debugger) =>
        match recv_time {
          Some(time) => core_debugger.update_elapsed(time),
          None => ()
        }
      None => ()
    }
    data
  }

  fn send(&mut self, ch_option : Self::ChannelOption, data : T) {
    self.debug_send(ch_option, data, &mut None)
  }
  fn recv(&mut self, ch_option : Self::ChannelOption) -> T{
    self.debug_recv(ch_option, &mut None)
  }

  fn get_row(&self) -> usize {
    self.row
  }

  fn get_col(&self) -> usize {
    self.col
  }
}

pub trait NetworkBuilder<T:Sendable> {
  type CoreType: CoreInfo<T>;
  fn build(&self, rows: usize, cols : usize) -> Vec<Self::CoreType>;
}

#[derive(Clone,Copy)]
pub struct TaurusNetworkBuilder {
  latency : Duration,
  bandwidth : usize,
  startup : Duration,
}

impl TaurusNetworkBuilder{
  pub fn new(latency : usize, bandwidth  : usize, startup : usize)
    -> Self { 
      TaurusNetworkBuilder {
        latency : if latency != 0 {Duration::new(0,latency as u32)} else {Duration::ZERO},
        bandwidth : if bandwidth == 0 {1} else {bandwidth},
        startup : if startup != 0 {Duration::new(0,startup as u32)} else {Duration::ZERO},
    }}
}

impl<T:Sendable> NetworkBuilder<T> for TaurusNetworkBuilder {
  type CoreType = TaurusCoreInfo<T>;

  fn build(&self, rows: usize, cols : usize) -> Vec<Self::CoreType> {
      let num_cores = cols * rows;
      let mut cores : Vec<TaurusCoreInfo<T>> = Vec::with_capacity(num_cores);
      for row in 0..rows {
        for col in 0..cols {
          cores.push(TaurusCoreInfo{ row, col, 
            core_comm : CoreComm::new(), 
            comm_info : CommInfo { latency: self.latency,
                                   bandwidth : self.bandwidth,
                                   startup : self.startup,
                                   broadcast_size: rows }
          })
        }
      }

    for i in 0..rows {
      let mut bchannels : Vec<Broadcast<(T, Option<Duration>)>> = Broadcast::new(cols);
      for step in 0..cols {
        let core_index = rows * i + step;
        cores[core_index].core_comm.row = bchannels.pop().unwrap();
      }
    }

    for i in 0..cols {
      let mut bchannels : Vec<Broadcast<(T, Option<Duration>)>> = Broadcast::new(rows);
      for step in 0..rows {
        let core_index = rows * step + i;
        cores[core_index].core_comm.col = bchannels.pop().unwrap();
      }
    }
    
    for i in 0..num_cores {
      let (up, down) = Direct::new();
      let up_index = i;
      let down_index = ( num_cores + i - cols ) % num_cores;

      cores[up_index].core_comm.up = up;
      cores[down_index].core_comm.down = down; 

      let (right, left) = Direct::new();
      let right_index = i;
      let left_index = i - ( i % cols ) + ( (i +  1) % cols );

      cores[right_index].core_comm.right = right;
      cores[left_index].core_comm.left = left; 
    }
    
    return cores
      
  }
}


pub struct Processor<H : Sendable + 'static, T : Sendable + 'static, CoreType: CoreInfo<T> + 'static> {
  pub rows : usize,
  pub cols : usize,
  networkbuilder : Box<dyn NetworkBuilder<T, CoreType = CoreType>>,
  handles : Vec<JoinHandle<(H, CoreDebug)>>,
  debugs : Vec<CoreDebug>
}

impl<H : Sendable + 'static, T : Sendable + 'static, CoreType: CoreInfo<T>> Processor<H, T, CoreType> {
  pub fn new(rows : usize, cols : usize, networkbuilder : Box<dyn NetworkBuilder<T, CoreType = CoreType>>)
    -> Processor<H, T, CoreType>{
    Processor {rows , cols , networkbuilder, handles : Vec::new(), debugs : Vec::new() }
  }

  pub fn build_network(&self) -> Vec<CoreType> {
    self.networkbuilder.build(self.rows, self.cols)
  }

  pub fn run_debug_core<F> (&mut self, f: F, mut core_info : CoreType) 
  where
      F: FnOnce(&mut CoreType, &mut CoreDebugger) -> H + Send + 'static,
  {
        let handle = thread::spawn(move || {
          let mut debugger = CoreDebugger::new(core_info.get_row(), core_info.get_col());
          let result = f(&mut core_info, &mut debugger);
          (result, debugger.end())
        });
        self.handles.push(handle);
  }

  pub fn run_core<F> (&mut self, f: F, core_info : CoreType) 
  where
      F: FnOnce(&mut CoreType) -> H + Send + 'static,
  {
      self.run_debug_core(|core_info , _| {
        f(core_info)
      }, core_info) 
  }

  pub fn collect_results (&mut self) -> Vec<H> {
    let mut results = Vec::new();
    while !self.handles.is_empty() {
      let handle = self.handles.pop().unwrap();
      let (result, debug) = handle.join().unwrap();
      self.debugs.push(debug);
      results.push(result);
    }
    results
  }

  pub fn display_debug_time (&self) {
    for debug in &self.debugs {
      println!("Core {} {} elapsed time: {}µs",
               debug.row, debug.col, debug.elapsed.as_micros());
    }
  }

  pub fn max_debug_time (&self) ->  Option<u128>{
    self.debugs.iter().map(|debug| debug.elapsed.as_micros()).max()
  }

  pub fn debug_direct_counts (&self) -> Vec<usize> {
    self.debugs.iter().map(|debug| debug.direct_count).collect()
  }

  pub fn debug_broadcast_counts (&self) -> Vec<usize> {
    self.debugs.iter().map(|debug| debug.broadcast_count).collect()
  }
}

#[derive(Copy,Clone,Debug, PartialEq)]
pub struct SubmatrixDim {
  pub start_row : usize,
  pub start_col : usize,
  pub width : usize,
  pub height : usize,
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

pub fn get_submatrices_dim(processor_rows : usize, processor_cols : usize, matrix_rows : usize, matrix_cols : usize) -> Vec<SubmatrixDim> {
  let dim_along_y = get_submatrices_dim_along_axis(processor_rows, matrix_rows);
  let dim_along_x = get_submatrices_dim_along_axis(processor_cols, matrix_cols);

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

pub fn get_submatrices<K: Clone>(processor_rows : usize, processor_cols : usize, matrix : &Vec<Vec<K>>) -> Vec<Vec<Vec<K>>> {
  let matrix_rows = matrix.len();
  let matrix_cols = matrix[0].len();

  let submatrices_dim = get_submatrices_dim(processor_rows, processor_cols, matrix_rows, matrix_cols);
  
  get_matrix_slices(matrix, &submatrices_dim)
}


#[cfg(test)]
mod tests;
