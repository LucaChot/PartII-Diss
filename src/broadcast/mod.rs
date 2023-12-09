use std::fmt::Display;
use std::sync::mpsc::{self, RecvError};
use std::sync::{Arc, Mutex};

pub struct BChannel<T : Clone + Display> {
  rx : mpsc::Receiver<T>,
  txs : Vec<mpsc::Sender<T>>,
  brlock : Arc<Mutex<i32>>,
}

impl<T : Clone + Display> BChannel<T> {
  pub fn send(&self, data : T)  -> () {
    let cloned_senders = self.txs.clone();
    let _ = self.brlock.lock();
    for tx in cloned_senders {
      tx.send(data.clone()).unwrap();
    }
  }

  pub fn recv(&self) -> Result<T, RecvError> {
    self.rx.recv()
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
      })
    }

    return bchannels;
  }

  pub fn empty() -> BChannel<T> {
    BChannel {
      rx : mpsc::channel().1,
      txs : Vec::with_capacity(0),
      brlock : Arc::new(Mutex::new(0)),
    }
  }
}

#[cfg(test)]
mod tests;
