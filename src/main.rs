use std::{thread, sync::mpsc};
use std::fmt::Display;

mod broadcast;
mod processor;
use broadcast::BChannel;
use processor::hashtag_processor;

pub struct ProcessorInfo<T : Clone + Display> {
  row : usize,
  col : usize,
  row_broadcast : BChannel<T>,
  col_broadcast : BChannel<T>,
}

fn thread_matrix_mult(a : i32, b : i32, dim : usize, p_info : &ProcessorInfo<i32>) -> i32 {
  let mut c = 0;
  for iter in 0..dim {
    if p_info.row == iter {
      p_info.row_broadcast.send(a.clone());
    }
    if p_info.col == iter {
      p_info.col_broadcast.send(b.clone());
    }
    let received_a = p_info.row_broadcast.recv().unwrap();
    let received_b = p_info.col_broadcast.recv().unwrap();

    c += received_a * received_b; 
  }
  return c;
}
    

fn main() {
  const NUM_PROCESSORS: usize = 9;

  let mut processors : Vec<Vec<BChannel<i32>>> = hashtag_processor(3, 3);

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  let (main_tx, main_rx) = mpsc::channel();

  let a_matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

  let b_matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

  let n = b_matrix.len();
  for j in 0..n {
    for i in 0..n {
      let mut bchannels = processors.pop().unwrap();
      let col_bchannel = bchannels.pop().unwrap();
      let row_bchannel = bchannels.pop().unwrap();
      let dim = b_matrix.len();
      
      let a = a_matrix[j][i];
      let b = b_matrix[j][i];
      
      let tx = main_tx.clone();

      let handle = thread::spawn(move || {
        let p_info = ProcessorInfo { 
          row: i,
          col: j, 
          row_broadcast: row_bchannel, 
          col_broadcast: col_bchannel
        };
        let c = thread_matrix_mult(a, b, dim, &p_info);
        tx.send((i, j, c)).unwrap();
      });
      handles.push(handle);
    }
  }

  drop(main_tx);

  let mut c_matrix: Vec<Vec<i32>> = vec![
    vec![0, 0, 0],
    vec![0, 0, 0],
    vec![0, 0, 0],
  ];
  for (i, j, c)  in main_rx {
    c_matrix[j][i] = c;
  }

  dbg!(c_matrix);

  for handle in handles {
    handle.join().unwrap();
  }

}
