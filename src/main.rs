use std::collections::VecDeque;
use std::{thread, sync::mpsc, f64};
use std::fmt::{Display,Formatter,Result};

mod broadcast;
mod processor;
use broadcast::BChannel;
use processor::{hashtag_processor, fox_otto_processor};


pub struct ProcessorInfo<T : Clone + Display> {
  row : usize,
  col : usize,
  row_broadcast : BChannel<T>,
  col_broadcast : BChannel<T>,
}

pub struct FoxOttoProcessorInfo<T : Clone + Display> {
  row : usize,
  col : usize,
  row_broadcast : BChannel<T>,
  tx : mpsc::Sender<T>,
  rx : mpsc::Receiver<T>,
}

#[derive(Clone)]
struct Msg {
  w : i32,
  p : i32,
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.w, self.p)
    }
}

fn thread_matrix_mult(m : Msg, dim : usize, p_info : &ProcessorInfo<Msg>) -> Msg {
  let mut next_m = m.clone();
  for iter in 0..dim {
    if p_info.row == iter {
      p_info.row_broadcast.send(m.clone());
    }
    if p_info.col == iter {
      p_info.col_broadcast.send(m.clone());
    }
    let received_a = p_info.row_broadcast.recv().unwrap();
    let received_b = p_info.col_broadcast.recv().unwrap();

    if received_a.w != -1 && received_b.w != -1 && ( next_m.w == -1 || received_a.w + received_b.w < next_m.w ){
      next_m.w = received_a.w + received_b.w;
      next_m.p = received_b.p;
    }
  }
  return next_m;
}
    
fn fox_otto_matrix_mult(m : Msg, dim : usize, p_info : &FoxOttoProcessorInfo<Msg>) -> Msg {
  let mut next_m = m.clone();
  let mut curr_b = m.clone();
  for iter in 0..dim {
    if p_info.row == (( iter + p_info.col ) % dim ) {
      p_info.row_broadcast.send(m.clone());
    }
    let received_a = p_info.row_broadcast.recv().unwrap();

    if received_a.w != -1 && curr_b.w != -1 && ( next_m.w == -1 || received_a.w + curr_b.w < next_m.w ){
      next_m.w = received_a.w + curr_b.w;
      next_m.p = curr_b.p;
    }

    let _ = p_info.tx.send(curr_b);
    curr_b = p_info.rx.recv().unwrap();
  }
  return next_m;
}

fn main() {
  // P matrix
  let p_matrix: Vec<Vec<i32>> = vec![
        vec![1,1,1,1,5,6,7],
        vec![1,2,3,4,2,6,7],
        vec![1,2,3,4,5,3,3],
        vec![1,2,3,4,5,6,4],
        vec![1,2,3,4,5,6,7],
        vec![1,6,3,4,5,6,7],
        vec![1,2,3,4,5,6,7],
    ];

  // W matrix
  let w_matrix: Vec<Vec<i32>> = vec![
        vec![ 0, 6, 2, 3,-1,-1,-1],
        vec![-1, 0,-1,-1, 1,-1,-1],
        vec![-1,-1, 0,-1,-1, 2, 1],
        vec![-1,-1,-1, 0,-1,-1, 2],
        vec![-1,-1,-1,-1, 0,-1,-1],
        vec![-1, 1,-1,-1,-1, 0,-1],
        vec![-1,-1,-1,-1,-1,-1, 0],
    ];

  // Dimensions of matrix
  let dim = p_matrix.len();
  // Number of matrix squaring that needs to be done
  let iterations = f64::ceil(f64::log2(dim as f64)) as usize;

  // Thread per element in matrix
  let num_processors: usize = dim * dim;

  // Messaging channels for each thread
  let mut processors : VecDeque<(BChannel<Msg>, mpsc::Sender<Msg>, mpsc::Receiver<Msg>)> = VecDeque::from(fox_otto_processor(dim, dim));

  let mut handles = Vec::with_capacity(num_processors);
  // Message channel to return values from each thread
  let (main_tx, main_rx) = mpsc::channel();


  for j in 0..dim {
    for i in 0..dim {
      // Assign each thread its corresponding channels
      let (row_broadcast, tx, rx) = processors.pop_front().unwrap();

      // Assign each threads matrix component
      let w = w_matrix[j][i];
      let p = p_matrix[j][i];
      
      // Sender for returning the results
      let result_tx = main_tx.clone();

      let handle = thread::spawn(move || {
        // Processor information
        let p_info = FoxOttoProcessorInfo { 
          row: i,
          col: j, 
          row_broadcast, 
          tx,
          rx
        };
        // Msg struct
        let mut m = Msg {
          w,
          p,
        };
        // Square the W matrix and update P
        for _ in 0..iterations {
          m = fox_otto_matrix_mult(m, dim, &p_info);
        }
        // Return the final values for the W and P matrix as well as the
        // index of the core so that main thread knows the values corresponding
        // location
        result_tx.send((i, j, m)).unwrap();
      });
      handles.push(handle);
    }
  }

  // Ensures that channel to main thread is closed when the other threads 
  // finish
  drop(main_tx);

  let mut next_w_matrix: Vec<Vec<i32>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
  ];
  let mut next_p_matrix: Vec<Vec<i32>> = vec![
    vec![1,1,1,1,5,6,7],
    vec![1,2,3,4,2,6,7],
    vec![1,2,3,4,5,3,3],
    vec![1,2,3,4,5,6,4],
    vec![1,2,3,4,5,6,7],
    vec![1,6,3,4,5,6,7],
    vec![1,2,3,4,5,6,7],
  ];

  // Assign the final values to the W and P matrix
  for (i, j, c)  in main_rx {
    next_w_matrix[j][i] = c.w;
    next_p_matrix[j][i] = c.p;
  }

  dbg!(next_w_matrix);
  println!("-----------------------------");
  dbg!(next_p_matrix);

  for handle in handles {
    handle.join().unwrap();
  }

}
