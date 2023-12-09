use std::{thread, sync::mpsc};

mod broadcast;
mod processor;
use broadcast::BChannel;
use processor::hashtag_processor;

fn main() {
  const NUM_PROCESSORS: usize = 4;

  let mut processors : Vec<Vec<BChannel<i32>>> = hashtag_processor(2, 2);

  let mut handles = Vec::with_capacity(NUM_PROCESSORS);
  let (main_tx, main_rx) = mpsc::channel();

  let a_matrix: Vec<Vec<i32>> = vec![
        vec![1, 2],
        vec![3, 4],
    ];

  let b_matrix: Vec<Vec<i32>> = vec![
        vec![1, 2],
        vec![3, 4],
    ];

  for j in 0..2 {
    for i in 0..2 {
      let mut bchannels = processors.pop().unwrap();
      let col_bchannel = bchannels.pop().unwrap();
      let row_bchannel = bchannels.pop().unwrap();
      
      let a = a_matrix[j][i];
      let b = b_matrix[j][i];
      
      let tx = main_tx.clone();

      let handle = thread::spawn(move || {
        let mut c = 0;
        for iter in 0..2 { 
          if i == iter {
            row_bchannel.send(a.clone());
          }
          if j == iter {
            col_bchannel.send(b.clone());
          }

          let received_a = row_bchannel.recv().unwrap();
          let received_b = col_bchannel.recv().unwrap();

          c += received_a * received_b; 
        }
        tx.send((i, j, c)).unwrap();
      });
      handles.push(handle);
    }
  }

  drop(main_tx);

  let mut c_matrix: Vec<Vec<i32>> = vec![
    vec![0, 0],
    vec![0, 0],
  ];
  for (i, j, c)  in main_rx {
    c_matrix[j][i] = c;
  }

  dbg!(c_matrix);

  for handle in handles {
    handle.join().unwrap();
  }

}
