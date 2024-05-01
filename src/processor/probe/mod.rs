use std::time::Duration;
use cpu_time::ThreadTime;
use std::marker::PhantomData;

use crate::broadcast::Sendable;

use super::{Core, TimedCore};

#[derive(Clone, Debug)]
pub struct CoreDebug<T> {
  pub row : usize,
  pub col : usize,
  pub stat : T
}

impl<T : Clone + Sendable> CoreDebug<T> {
  pub fn new(row : usize, col : usize, stat : T) -> Self {
    CoreDebug { 
      row,
      col,
      stat,
    }
  }
}

impl<T : Sendable> Sendable for CoreDebug<T> {}

pub struct ThreadTimeProbe {
  core_debug : CoreDebug<Duration>,
  thread_time : ThreadTime,
  additional : Duration,
}

impl ThreadTimeProbe {
  pub fn new(row : usize, col : usize) -> Self {
    ThreadTimeProbe { 
      core_debug: CoreDebug::new(row, col, Duration::ZERO),
      thread_time: ThreadTime::now(),
      additional: Duration::ZERO
    }
  }

  pub fn get_curr_elapsed(&self) -> Duration {
    self.thread_time.elapsed() + self.additional
  }

  pub fn update_elapsed(&mut self, outer : Duration) {
    let current = self.get_curr_elapsed();
    if current < outer {
      self.additional += outer - current
    }
  }

  pub fn increment_time(&mut self, increment : Duration) {
    self.additional += increment;
  }

  pub fn end(mut self) -> CoreDebug<Duration>{
    self.core_debug.stat = self.get_curr_elapsed();
    self.core_debug
  }
}

pub trait Prober<D,U, CoreType> 
  where U : Sendable,
        CoreType : Core<U> 
{
  fn new(core : CoreType) -> Self;

  fn extract_stat(self) -> CoreDebug<D>;
}

pub struct ThreadTimeProber <T : Sendable, CoreType>
  where T : Sendable,
        CoreType : TimedCore<(T,Duration)>, 
{
  core : CoreType,
  probe : ThreadTimeProbe,
  phantom : PhantomData<T>,
} 

impl<T, CoreType> Core<T> for ThreadTimeProber<T, CoreType> 
  where T : Sendable,
        CoreType : TimedCore<(T,Duration)> 
{
    type ChannelOption= CoreType::ChannelOption;

    fn row(&self) -> usize {
        self.core.row()
    }

    fn col(&self) -> usize {
        self.core.col()
    }

    fn send(&mut self, data : T, ch_option : &Self::ChannelOption) {
      let comm_cost = self.core.transmission_time(&data, &ch_option);
      self.probe.increment_time(comm_cost);
      let recv_time =  self.probe.get_curr_elapsed() + self.core.latency();
      self.core.send((data,recv_time), &ch_option)
    }

    fn recv(&mut self, ch_option : &Self::ChannelOption) -> T {
      let (data, recv_time) = self.core.recv(ch_option);
      self.probe.update_elapsed(recv_time);
      data
    }
}


impl<T, CoreType> Prober<Duration, (T,Duration), CoreType> for ThreadTimeProber<T, CoreType> 
  where T : Sendable,
        CoreType : TimedCore<(T,Duration)> 
{
    fn new(core : CoreType) -> Self {
      let row = core.row();
      let col = core.col();
        ThreadTimeProber { core, probe: ThreadTimeProbe::new(row, col), phantom: PhantomData}
    }

    fn extract_stat(self) -> CoreDebug<Duration> {
        self.probe.end()
    }
}

#[cfg(test)]
mod tests;
