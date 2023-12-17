use std::{thread, sync::mpsc, f64};
use std::fmt::{Display,Formatter,Result};

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
  let mut processors : Vec<Vec<BChannel<Msg>>> = hashtag_processor(dim, dim);

  let mut handles = Vec::with_capacity(num_processors);
  // Message channel to return values from each thread
  let (main_tx, main_rx) = mpsc::channel();


  for j in 0..dim {
    for i in 0..dim {
      // Assign each thread its corresponding channels
      let mut bchannels = processors.pop().unwrap();
      let col_bchannel = bchannels.pop().unwrap();
      let row_bchannel = bchannels.pop().unwrap();

      // Assign each threads matrix component
      let w = w_matrix[j][i];
      let p = p_matrix[j][i];
      
      // Sender for returning the results
      let tx = main_tx.clone();

      let handle = thread::spawn(move || {
        // Processor information
        let p_info = ProcessorInfo { 
          row: i,
          col: j, 
          row_broadcast: row_bchannel, 
          col_broadcast: col_bchannel
        };
        // Msg struct
        let mut m = Msg {
          w,
          p,
        };
        // Square the W matrix and update P
        for _ in 0..iterations {
          m = thread_matrix_mult(m, dim, &p_info);
        }
        // Return the final values for the W and P matrix as well as the
        // index of the core so that main thread knows the values corresponding
        // location
        tx.send((i, j, m)).unwrap();
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

  for handle in handles {
    handle.join().unwrap();
  }

}
