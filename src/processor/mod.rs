use crate::broadcast::Sendable;
use std::{time::Duration, thread::{JoinHandle, self}, marker::PhantomData};

pub mod taurus;
pub mod probe;

use self::probe::{Prober, CoreDebug};


pub trait TimedCore<T : Sendable> : Core<T> {
  fn blank() -> Self;
  fn transmission_time<S>(&self, item : &S ,ch_option : &Self::ChannelOption) -> Duration;
  fn latency(&self) -> Duration;
}

pub trait Core<T : Sendable> {
  type ChannelOption;

  fn row(&self) -> usize;
  fn col(&self) -> usize;
  fn send(&mut self, data : T, ch_option : &Self::ChannelOption);
  fn recv(&mut self, ch_option : &Self::ChannelOption) -> T;
}


pub trait NetworkBuilder<T:Sendable> {
  type CoreType: Core<T>;
  fn build(&self, rows: usize, cols : usize) -> Vec<Self::CoreType>;
}


pub struct Processor<H, T, CoreType> 
  where H : Sendable + 'static,
        T : Sendable + 'static,
        CoreType : Core<T> + Send,
        {
  pub rows : usize,
  pub cols : usize,
  cores : Vec<CoreType>,
  handles : Vec<JoinHandle<H>>,
  phantom : PhantomData<T>,
}

impl<H, T, CoreType> Processor<H, T, CoreType> 
  where H : Sendable + 'static,
        T : Sendable + 'static,
        CoreType : Core<T> + Send + 'static,
        {
  pub fn new(rows : usize, cols : usize, networkbuilder : impl NetworkBuilder<T, CoreType = CoreType>)
    -> Self {
    Processor {rows , cols, handles : Vec::new(), cores : networkbuilder.build(rows, cols), phantom : PhantomData}
  }

  pub fn run_core<F> (&mut self, f: F) 
  where
      F: FnOnce(&mut CoreType) -> H + Send + 'static,
  {
    match self.cores.pop() {
      None => (),
      Some(mut core_info) => {
        let handle = thread::spawn(move || {
          let result = f(&mut core_info);
          result
        });
        self.handles.push(handle);
      }
    }
  }

  pub fn collect_results (&mut self) -> Vec<H> {
    let mut results = Vec::new();
    while !self.handles.is_empty() {
      let handle = self.handles.pop().unwrap();
      results.push(handle.join().unwrap());
    }
    results
  }
}


pub struct ProbeProcessor<D, H, U, CoreType> 
  where D : Sendable + 'static,
        H : Sendable + 'static,
        U : Sendable + 'static,
        CoreType : Core<U> + Send,
        {
  proc : Processor<(H, CoreDebug<D>),U,CoreType>,
  debugs : Vec<CoreDebug<D>>,
}

impl<D, H, U, CoreType> ProbeProcessor<D, H, U, CoreType> 
  where D : Sendable + 'static,
        H : Sendable + 'static,
        U : Sendable + 'static,
        CoreType : Core<U> + Send + 'static,
        {
  pub fn new(rows : usize, cols : usize, networkbuilder : impl NetworkBuilder<U, CoreType = CoreType>)
    -> Self {
    ProbeProcessor {
      proc : Processor::new(rows , cols , networkbuilder),
      debugs : Vec::new()
    }
  }

  pub fn rows(&self) -> usize {
    self.proc.rows
  }

  pub fn cols(&self) -> usize {
    self.proc.cols
  }

  pub fn run_core<F,P> (&mut self, f: F) 
  where
      P : Prober<D,U,CoreType>,
      F: FnOnce(&mut P) -> H + Send + 'static,
  {
    match self.proc.cores.pop() {
      None => (),
      Some(core_info) => {
        let handle = thread::spawn(move || {
          let mut probe = P::new(core_info);
          let result = f(&mut probe);
          (result, probe.extract_stat())
        });
        self.proc.handles.push(handle);
      }
    }
  }

  pub fn collect_results (&mut self) -> Vec<H> {
    let results = self.proc.collect_results();
    let mut data = Vec::new();
    for (result, debug) in results.into_iter(){
      self.debugs.push(debug);
      data.push(result);
    }
    data
  }

  pub fn debug_stats(&self) -> &Vec<CoreDebug<D>> {
    &self.debugs
  }
}

impl<H, T, CoreType> ProbeProcessor<Duration, H, T, CoreType> 
  where H : Sendable + 'static,
        T : Sendable + 'static,
        CoreType : TimedCore<T> + Send,
        {

  pub fn display_debug_time (&self) {
    for debug in &self.debugs {
      println!("Core {} {} elapsed time: {}µs",
               debug.row, debug.col, debug.stat.as_micros());
    }
  }

  pub fn max_debug_time (&self) ->  Option<u128>{
    self.debugs.iter().map(|debug| debug.stat.as_micros()).max()
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
