use crate::broadcast::{Broadcast, Sendable, Direct, Channel};
use std::{time::Duration, mem::size_of_val, ops::{Mul, Div}};

use super::{Core, TimedCore, NetworkBuilder};

struct TaurusComm<T:Sendable>{
  left : Direct<T>,
  right : Direct<T>,
  up : Direct<T>,
  down : Direct<T>,
  row : Broadcast<T>,
  col : Broadcast<T>,
}

impl<T : Sendable> TaurusComm<T> {
  fn new() -> TaurusComm<T> {
    TaurusComm { 
      left: Direct::empty(),
      right: Direct::empty(),
      up: Direct::empty(),
      down: Direct::empty(),
      row: Broadcast::empty(),
      col: Broadcast::empty()
    }
  } 
}

pub struct TimedTaurusCore<T : Sendable> {
  latency : Duration,
  bandwidth : usize,
  startup : Duration,
  broadcast_size : usize,
  core : TaurusCore<T>
}

impl<T : Sendable> TimedTaurusCore<T> {
  pub fn new(latency: Duration, bandwidth: usize, startup: Duration, broadcast_size: usize, core: TaurusCore<T>) -> Self { 
    Self { latency, bandwidth, startup, broadcast_size, core } 
  }
} 

impl<T: Sendable> Core<T> for TimedTaurusCore<T> {
    type ChannelOption = TaurusOption;

    fn row(&self) -> usize {
      self.core.row()
    }

    fn col(&self) -> usize {
      self.core.col()
    }

    fn send(&mut self, data : T, ch_option : &Self::ChannelOption) {
      self.core.send(data,ch_option)
    }

    fn recv(&mut self, ch_option : &Self::ChannelOption) -> T {
      self.core.recv(ch_option)
    }
}

impl<T : Sendable> TimedCore<T> for TimedTaurusCore<T> {

  fn transmission_time<S>(&self, item : &S ,ch_option : &Self::ChannelOption) -> Duration {
    Duration::new(size_of_val(item) as u64,0).div(self.bandwidth as u32) + 
     match  ch_option {
      TaurusOption::ROW | TaurusOption::COL =>  self.startup.mul(self.broadcast_size as u32),
      _ => Duration::ZERO
    }
  }

  fn latency(&self) -> Duration {
    self.latency
  }

  fn blank() -> Self {
    TimedTaurusCore { latency: Duration::ZERO, bandwidth: 1, startup: Duration::ZERO, broadcast_size : 1, core : TaurusCore::new(0,0)}
  }
}

pub enum TaurusOption {
  LEFT,
  RIGHT,
  UP,
  DOWN,
  ROW,
  COL,
}

pub struct TaurusCore<T : Sendable> {
  pub row : usize,
  pub col : usize,
  core_comm : TaurusComm<T>,
} 

impl<T:Sendable> TaurusCore<T> {
  pub fn new(row : usize, col : usize) -> Self {
    TaurusCore{ row, col, 
                core_comm : TaurusComm::new(), 
    }
  }
}

impl<T:Sendable> Core<T> for TaurusCore<T> {
  type ChannelOption = TaurusOption;

  fn send(&mut self, data : T, ch_option : &Self::ChannelOption){
    match ch_option {
      TaurusOption::LEFT => self.core_comm.left.send(data),
      TaurusOption::RIGHT => self.core_comm.right.send(data),
      TaurusOption::UP => self.core_comm.up.send(data),
      TaurusOption::DOWN => self.core_comm.down.send(data),
      TaurusOption::ROW => self.core_comm.row.send(data),
      TaurusOption::COL => self.core_comm.col.send(data),
    }
  }

  fn recv(&mut self, ch_option : &Self::ChannelOption) -> T{
    let data = match ch_option {
      TaurusOption::LEFT => self.core_comm.left.recv(),
      TaurusOption::RIGHT => self.core_comm.right.recv(),
      TaurusOption::UP => self.core_comm.up.recv(),
      TaurusOption::DOWN => self.core_comm.down.recv(),
      TaurusOption::ROW => self.core_comm.row.recv(),
      TaurusOption::COL => self.core_comm.col.recv(),
    };
    data
  }

  fn row(&self) -> usize {
    self.row
  }

  fn col(&self) -> usize {
    self.col
  }
}

#[derive(Clone,Copy)]
pub struct TaurusNetworkBuilder;

impl<T:Sendable> NetworkBuilder<T> for TaurusNetworkBuilder {
  type CoreType = TaurusCore<T>;

  fn build(&self, rows: usize, cols : usize) -> Vec<Self::CoreType> {
      let num_cores = cols * rows;
      let mut cores : Vec<TaurusCore<T>> = Vec::with_capacity(num_cores);
      for row in 0..rows {
        for col in 0..cols {
          cores.push(TaurusCore::new(row, col))
        }
      }

    for i in 0..rows {
      let mut bchannels : Vec<Broadcast<T>> = Broadcast::new(cols);
      for step in 0..cols {
        let core_index = rows * i + step;
        cores[core_index].core_comm.row = bchannels.pop().unwrap();
      }
    }

    for i in 0..cols {
      let mut bchannels : Vec<Broadcast<T>> = Broadcast::new(rows);
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
    
    cores.reverse();
    return cores
  }
}

#[derive(Clone,Copy)]
pub struct TimeTaurusNetworkBuilder {
  latency : Duration,
  bandwidth : usize,
  startup : Duration,
  networkbuilder : TaurusNetworkBuilder
}

impl TimeTaurusNetworkBuilder{
  pub fn new(latency : usize, bandwidth  : usize, startup : usize)
    -> Self { 
      TimeTaurusNetworkBuilder {
        latency : if latency != 0 {Duration::new(0,latency as u32)} else {Duration::ZERO},
        bandwidth : if bandwidth == 0 {1} else {bandwidth},
        startup : if startup != 0 {Duration::new(0,startup as u32)} else {Duration::ZERO},
        networkbuilder : TaurusNetworkBuilder
    }}
}

impl<T:Sendable> NetworkBuilder<T> for TimeTaurusNetworkBuilder {
  type CoreType = TimedTaurusCore<T>;

  fn build(&self, rows: usize, cols : usize) -> Vec<Self::CoreType> {
    let cores = self.networkbuilder.build(rows, cols); 
    cores.into_iter()
      .map(|core| TimedTaurusCore::new(self.latency, self.bandwidth, self.startup, rows, core))
      .collect()
  }
}

