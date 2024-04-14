use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;


pub trait Sendable : Clone + Debug + std::marker::Send{}

pub trait Channel<T:Sendable> {
  fn send(&self, data : T);
  fn recv(&self) -> T;
}

pub struct Broadcast<T : Sendable> {
  rx : mpsc::Receiver<T>,
  txs : Arc<Mutex<Vec<mpsc::Sender<T>>>>,
}

impl<T : Sendable> Broadcast<T> {
  pub fn new(n : usize) -> Vec<Broadcast<T>> {
    let mut txs : Vec<mpsc::Sender<T>> = Vec::with_capacity(n);
    let mut rxs : Vec<mpsc::Receiver<T>> = Vec::with_capacity(n);

    for _ in 0..n {
      let (tx, rx) = mpsc::channel();
      txs.push(tx);
      rxs.push(rx);
    }

    let mut bchannels = Vec::with_capacity(n);

    let ref_txs = Arc::new(Mutex::new(txs));
    for i in 0..n {
      bchannels.push(Broadcast {
        rx : std::mem::replace(&mut rxs[i], mpsc::channel().1,) ,
        txs : Arc::clone(&ref_txs),
      })
    }

    return bchannels;
  }

  pub fn empty() -> Broadcast<T> {
    Broadcast {
      rx : mpsc::channel().1,
      txs : Arc::new(Mutex::new(Vec::with_capacity(0))),
    }
  }
}

impl<T:Sendable> Channel<T> for Broadcast<T> {
  fn send(&self, data : T) {
    let txs = self.txs.lock().unwrap();
    for tx in &(*txs) {
      tx.send(data.clone()).unwrap();
    }
  }

  fn recv(&self) -> T {
     self.rx.recv().unwrap()
  }
}

pub struct Direct<T : Sendable> {
  rx : mpsc::Receiver<T>,
  tx : mpsc::Sender<T>,
}

impl<T : Sendable> Direct<T> {

  pub fn new() -> (Direct<T>, Direct<T>) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (
      Direct { tx: tx1, rx : rx2 },
      Direct { tx : tx2, rx : rx1 }
    )
  }

  pub fn empty() -> Direct<T> {
    let (tx, rx) = mpsc::channel();
    Direct { tx, rx }
  }
}

impl<T:Sendable> Channel<T> for Direct<T> {
  fn send(&self, data : T)  -> () {
    self.tx.send(data).unwrap();
  }

  fn recv(&self) -> T {
    self.rx.recv().unwrap()
  }
}
#[cfg(test)]
mod tests;
