use std::fmt::Display;
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

#[cfg(test)]
mod tests;
