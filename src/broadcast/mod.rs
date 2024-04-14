use std::sync::mpsc;
use core::cell::RefCell;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;

use crate::processor::CoreDebug;

pub trait Sendable : Clone + Debug + std::marker::Send{}

pub struct BChannel<T : Sendable> {
  rx : mpsc::Receiver<(T, Duration)>,
  txs : Arc<Mutex<Vec<mpsc::Sender<(T, Duration)>>>>,
  sent : RefCell<usize>,
  core_debug : Arc<Mutex<RefCell<CoreDebug>>>,
}

impl<T : Sendable> BChannel<T> {
  pub fn send(&self, data : T)  -> () {
    let txs = self.txs.lock().unwrap();
    let core_debug_mut = self.core_debug.lock().unwrap();
    let core_debug = core_debug_mut.borrow_mut();
    let elapsed =  core_debug.get_elapsed();
    for tx in &(*txs) {
      tx.send((data.clone(), elapsed)).unwrap();
    }
    let mut sent = self.sent.borrow_mut();
    *sent += 1;
  }

  pub fn recv(&self) -> T {
    let core_debug_mut = self.core_debug.lock().unwrap();
    let mut core_debug = core_debug_mut.borrow_mut();
    let (data, outer_time) = self.rx.recv().unwrap();
    core_debug.update_elapsed(outer_time);
    data
  }

  pub fn new(n : usize) -> Vec<BChannel<T>> {
    let mut txs : Vec<mpsc::Sender<(T, Duration)>> = Vec::with_capacity(n);
    let mut rxs : Vec<mpsc::Receiver<(T, Duration)>> = Vec::with_capacity(n);

    for _ in 0..n {
      let (tx, rx) = mpsc::channel();
      txs.push(tx);
      rxs.push(rx);
    }

    let mut bchannels = Vec::with_capacity(n);

    let ref_txs = Arc::new(Mutex::new(txs));
    for i in 0..n {
      bchannels.push(BChannel {
        rx : std::mem::replace(&mut rxs[i], mpsc::channel().1,) ,
        txs : Arc::clone(&ref_txs),
        sent : RefCell::new(0),
        core_debug : Arc::new(Mutex::new(RefCell::new(CoreDebug::new())))
      })
    }

    return bchannels;
  }

  pub fn empty() -> BChannel<T> {
    BChannel {
      rx : mpsc::channel().1,
      txs : Arc::new(Mutex::new(Vec::with_capacity(0))),
      sent : RefCell::new(0),
      core_debug : Arc::new(Mutex::new(RefCell::new(CoreDebug::new())))
    }
  }

  pub fn get_sent(&self) -> usize {
    *self.sent.borrow()
  }

  pub fn set_core_debug(&mut self, new : Arc<Mutex<RefCell<CoreDebug>>>) {
    self.core_debug = new;
  }
}

pub struct Channel<T : Sendable> {
  rx : mpsc::Receiver<(T, Duration)>,
  tx : mpsc::Sender<(T, Duration)>,
  sent : RefCell<usize>,
  core_debug : Arc<Mutex<RefCell<CoreDebug>>>,
}

impl<T : Sendable> Channel<T> {
  pub fn send(&self, data : T)  -> () {
    let core_debug_mut = self.core_debug.lock().unwrap();
    let core_debug = core_debug_mut.borrow_mut();
    self.tx.send((data.clone(), core_debug.get_elapsed())).unwrap();
    let mut sent = self.sent.borrow_mut();
    *sent += 1;
  }

  pub fn recv(&self) -> T {
    let core_debug_mut = self.core_debug.lock().unwrap();
    let mut core_debug = core_debug_mut.borrow_mut();

    let (data, outer_time) = self.rx.recv().unwrap();
    core_debug.update_elapsed(outer_time);
    data
  }

  pub fn new() -> (Channel<T>, Channel<T>) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (
      Channel { tx: tx1, rx : rx2, 
        sent : RefCell::new(0), 
        core_debug : Arc::new(Mutex::new(RefCell::new(CoreDebug::new())))
      },
      Channel { tx : tx2, rx : rx1,
        sent : RefCell::new(0), 
        core_debug : Arc::new(Mutex::new(RefCell::new(CoreDebug::new())))
      }
    )
  }

  pub fn empty() -> Channel<T> {
    let (tx, rx) = mpsc::channel();
    Channel { tx, rx, sent : RefCell::new(0), core_debug : Arc::new(Mutex::new(RefCell::new(CoreDebug::new())))
}
  }

  pub fn get_sent(&self) -> usize {
    *self.sent.borrow()
  }

  pub fn set_core_debug(&mut self, new : Arc<Mutex<RefCell<CoreDebug>>>) {
    self.core_debug = new;
  }
}
#[cfg(test)]
mod tests;
