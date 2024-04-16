use std::time::Duration;
use cpu_time::ThreadTime;

#[derive(Clone)]
pub struct CoreDebug {
  pub row : usize,
  pub col : usize,
  pub direct_count : usize,
  pub broadcast_count : usize,
  pub elapsed : Duration,
  pub additional : Duration,
}

pub struct CoreDebugger {
  core_debug : CoreDebug,
  thread_time : ThreadTime,
}

impl CoreDebugger {
  pub fn new(row : usize, col : usize) -> Self {
    CoreDebugger { core_debug: CoreDebug::new(row, col), thread_time: ThreadTime::now() }
  }

  pub fn get_curr_elapsed(&self) -> Duration {
    self.thread_time.elapsed() + self.core_debug.additional
  }

  pub fn update_elapsed(&mut self, outer : Duration) {
    let current = self.get_curr_elapsed();
    if current < outer {
      self.core_debug.additional = outer - self.thread_time.elapsed()
    }
  }

  pub fn increment_direct(&mut self) {
    self.core_debug.direct_count += 1;
  }

  pub fn increment_broadcast(&mut self) {
    self.core_debug.broadcast_count += 1;
  }
  
  pub fn end(mut self) -> CoreDebug {
    self.core_debug.elapsed = self.get_curr_elapsed();
    self.core_debug
  }
}

impl CoreDebug {
  pub fn new(row : usize, col : usize) -> CoreDebug {
    CoreDebug { 
      row,
      col,
      direct_count : 0,
      broadcast_count : 0,
      elapsed : Duration::ZERO,
      additional : Duration::ZERO,
    }
  }
}
