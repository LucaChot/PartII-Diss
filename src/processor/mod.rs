use std::{fmt::Display, sync::mpsc::{Receiver, Sender, self}};
use super::BChannel;

/// This function returns a 2D Vec of BChannel<T> representing the broadcast 
/// channels for a hashtag processor (only vertical and horizontal broadcasts)
///
/// # Arguemnts
/// * `rows` - Number of rows in the processor matrix
/// * `cols` - Number of columns in the processor matrix
///
/// # Returns
/// Returns Vec<Vec<BCHannel<T>>> which has out length equal to the number of 
/// cores in the processor and inner length equal to the number of channels 
/// accessible by the corresponding core. 
pub fn hashtag_processor<T: Clone + Display>(rows : usize, cols : usize) -> Vec<Vec<BChannel<T>>> {
  let num_processors = cols * rows;
  let mut processors : Vec<Vec<BChannel<T>>>= Vec::with_capacity(num_processors);
  for _ in 0..num_processors {
    processors.push(Vec::with_capacity(2));
  }

  for i in 0..rows {
    let mut bchannels : Vec<BChannel<T>> = BChannel::new(cols);
    for step in 0..cols {
      processors[rows * i + step].push(std::mem::replace(&mut bchannels[step], BChannel::empty()));
    }
  }
  
  for j in 0..cols {
    let mut bchannels : Vec<BChannel<T>> = BChannel::new(rows);
    for step in 0..rows {
      processors[step * cols + j].push(std::mem::replace(&mut bchannels[step], BChannel::empty()));
    }
  }
  return processors
}

pub fn fox_otto_processor<T: Clone + Display>(rows : usize, cols : usize) -> Vec<(BChannel<T>, Sender<T>, Receiver<T>)> {
  let num_processors = cols * rows;
  let mut processors : Vec<(BChannel<T>, Sender<T>, Receiver<T>)> = Vec::with_capacity(num_processors);
  for _ in 0..num_processors {
    let (tx, rx) = mpsc::channel();
    processors.push((
      BChannel::empty(),
      tx,
      rx));
  }

  for i in 0..rows {
    let mut bchannels : Vec<BChannel<T>> = BChannel::new(cols);
    for step in 0..cols {
      processors[rows * i + step].0 = std::mem::replace(&mut bchannels[step], BChannel::empty());
    }
  }
  
  for j in 0..num_processors {
    let (tx, mut rx) = mpsc::channel();
    processors[j].1 = tx.clone();
    processors[( num_processors + j - cols ) % num_processors].2 = std::mem::replace(&mut rx, mpsc::channel().1); 
  }
  return processors
}

#[cfg(test)]
mod tests;
