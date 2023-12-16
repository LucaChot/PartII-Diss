use std::{thread, sync::mpsc};
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

fn thread_matrix_mult(w : i32, p : i32, dim : usize, p_info : &ProcessorInfo<Msg>) -> Msg {
  let mut next_w = w;
  let mut next_p = p;
  for iter in 0..dim {
    let m = Msg {
      w,
      p
    };
    if p_info.row == iter {
      p_info.row_broadcast.send(m.clone());
    }
    if p_info.col == iter {
      p_info.col_broadcast.send(m.clone());
    }
    let received_a = p_info.row_broadcast.recv().unwrap();
    let received_b = p_info.col_broadcast.recv().unwrap();

    if received_a.w != -1 && received_b.w != -1 && ( next_w == -1 || received_a.w + received_b.w < next_w ){
      next_w = received_a.w + received_b.w;
      next_p = received_b.p;
    }
  }
  return Msg {
    w : next_w,
    p : next_p
  }
}
    

fn main() {
  let p_matrix: Vec<Vec<i32>> = vec![
        vec![1,1,1,1,5,6,7],
        vec![1,2,3,4,2,6,7],
        vec![1,2,3,4,5,3,3],
        vec![1,2,3,4,5,6,4],
        vec![1,2,3,4,5,6,7],
        vec![1,6,3,4,5,6,7],
        vec![1,2,3,4,5,6,7],
    ];

  let w_matrix: Vec<Vec<i32>> = vec![
        vec![ 0, 6, 2, 3,-1,-1,-1],
        vec![-1, 0,-1,-1, 1,-1,-1],
        vec![-1,-1, 0,-1,-1, 2, 1],
        vec![-1,-1,-1, 0,-1,-1, 2],
        vec![-1,-1,-1,-1, 0,-1,-1],
        vec![-1, 1,-1,-1,-1, 0,-1],
        vec![-1,-1,-1,-1,-1,-1, 0],
    ];

  let num_processors: usize = p_matrix.len() * w_matrix.len();

  let mut processors : Vec<Vec<BChannel<Msg>>> = hashtag_processor(p_matrix.len(), p_matrix.len());

  let mut handles = Vec::with_capacity(num_processors);
  let (main_tx, main_rx) = mpsc::channel();


  let n = w_matrix.len();
  for j in 0..n {
    for i in 0..n {
      let mut bchannels = processors.pop().unwrap();
      let col_bchannel = bchannels.pop().unwrap();
      let row_bchannel = bchannels.pop().unwrap();
      let dim = p_matrix.len();
      
      let w = w_matrix[j][i];
      let p = p_matrix[j][i];
      
      let tx = main_tx.clone();

      let handle = thread::spawn(move || {
        let p_info = ProcessorInfo { 
          row: i,
          col: j, 
          row_broadcast: row_bchannel, 
          col_broadcast: col_bchannel
        };
        let c = thread_matrix_mult(w, p, dim, &p_info);
        tx.send((i, j, c)).unwrap();
      });
      handles.push(handle);
    }
  }

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
  for (i, j, c)  in main_rx {
    next_w_matrix[j][i] = c.w;
  }

  dbg!(next_w_matrix);

  for handle in handles {
    handle.join().unwrap();
  }

}
