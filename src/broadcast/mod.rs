use std::sync::mpsc;
use core::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;

pub trait Sendable : Clone + Debug + std::marker::Send{}

pub struct BChannel<T : Sendable> {
  rx : mpsc::Receiver<T>,
  txs : Vec<mpsc::Sender<T>>,
  brlock : Arc<Mutex<i32>>,
  sent : RefCell<usize>,
}

impl<T : Sendable> BChannel<T> {
  pub fn send(&self, data : T)  -> () {
    let _ = self.brlock.lock();
    for tx in &self.txs {
      tx.send(data.clone()).unwrap();
    }
    let mut sent = self.sent.borrow_mut();
    *sent += 1;
  }

  pub fn recv(&self) -> T {
    self.rx.recv().unwrap()
  }

  pub fn new(n : usize) -> Vec<BChannel<T>> {
    let mut txs : Vec<mpsc::Sender<T>> = Vec::with_capacity(n);
    let mut rxs : Vec<mpsc::Receiver<T>> = Vec::with_capacity(n);

    for _ in 0..n {
      let (tx, rx) = mpsc::channel();
      txs.push(tx);
      rxs.push(rx);
    }

    let block = Arc::new(Mutex::new(0));

    let mut bchannels = Vec::with_capacity(n);

    for i in 0..n {
      bchannels.push(BChannel {
        rx : std::mem::replace(&mut rxs[i], mpsc::channel().1,) ,
        txs : txs.clone(),
        brlock : Arc::clone(&block),
        sent : RefCell::new(0),
      })
    }

    return bchannels;
  }

  pub fn empty() -> BChannel<T> {
    BChannel {
      rx : mpsc::channel().1,
      txs : Vec::with_capacity(0),
      brlock : Arc::new(Mutex::new(0)),
      sent : RefCell::new(0),
    }
  }

  pub fn get_sent(&self) -> usize {
    *self.sent.borrow()
  }
}

pub struct Channel<T : Sendable> {
  rx : mpsc::Receiver<T>,
  tx : mpsc::Sender<T>,
  sent : RefCell<usize>,
}

impl<T : Sendable> Channel<T> {
  pub fn send(&self, data : T)  -> () {
    self.tx.send(data.clone()).unwrap();
    let mut sent = self.sent.borrow_mut();
    *sent += 1;
  }

  pub fn recv(&self) -> T {
    self.rx.recv().unwrap()
  }

  pub fn new() -> (Channel<T>, Channel<T>) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (Channel { tx: tx1, rx : rx2, sent : RefCell::new(0)}, Channel { tx : tx2, rx : rx1, sent : RefCell::new(0)})
  }

  pub fn empty() -> Channel<T> {
    let (tx, rx) = mpsc::channel();
    Channel { tx, rx, sent : RefCell::new(0)}
  }

  pub fn get_sent(&self) -> usize {
    *self.sent.borrow()
  }
}
#[cfg(test)]
mod tests;
