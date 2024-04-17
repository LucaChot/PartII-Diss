use super::*;
use std::{thread::sleep, time::Instant};


#[test]
fn general_correct_length(){
  let processor : Processor<i32,i32, TaurusCoreInfo<i32>> = 
    Processor::new(2,2, Box::new(TaurusNetworkBuilder{}));
  let bchannels = processor.build_network();
  assert_eq!(bchannels.len(), 4);
}

#[test]
fn general_correct_connection(){
  let processor : Processor<i32,i32, TaurusCoreInfo<i32>> = 
    Processor::new(2,2, Box::new(TaurusNetworkBuilder{}));
  let mut cores = processor.build_network();
  // Check that horizontal broadcast works
  cores[0].send(Taurus::UP,1);
  assert_eq!(cores[2].recv(Taurus::DOWN), 1);

  cores[0].send(Taurus::RIGHT,2);
  assert_eq!(cores[1].recv(Taurus::LEFT), 2);

  cores[0].send(Taurus::DOWN,3);
  assert_eq!(cores[2].recv(Taurus::UP), 3);

  cores[0].send(Taurus::LEFT,4);
  assert_eq!(cores[1].recv(Taurus::RIGHT), 4);

  cores[3].send(Taurus::UP,1);
  assert_eq!(cores[1].recv(Taurus::DOWN), 1);

  cores[3].send(Taurus::RIGHT,2);
  assert_eq!(cores[2].recv(Taurus::LEFT), 2);

  // Check that vertical broadcast works
  cores[3].send(Taurus::DOWN,3);
  assert_eq!(cores[1].recv(Taurus::UP), 3);

  cores[3].send(Taurus::LEFT,4);
  assert_eq!(cores[2].recv(Taurus::RIGHT), 4);
}

#[test]
fn general_correct_broadcast(){
  let processor : Processor<i32,i32, TaurusCoreInfo<i32>> = 
    Processor::new(2,2, Box::new(TaurusNetworkBuilder{}));
  let mut cores = processor.build_network();

  // Check that horizontal broadcast works
  cores[0].send(Taurus::ROW,0);
  assert_eq!(cores[0].recv(Taurus::ROW), 0);
  assert_eq!(cores[1].recv(Taurus::ROW), 0);

  cores[3].send(Taurus::ROW,1);
  assert_eq!(cores[2].recv(Taurus::ROW), 1);
  assert_eq!(cores[3].recv(Taurus::ROW), 1);

  // Check that vertical broadcast works
  cores[2].send(Taurus::COL,2);
  assert_eq!(cores[0].recv(Taurus::COL), 2);
  assert_eq!(cores[2].recv(Taurus::COL), 2);

  cores[1].send(Taurus::COL,3);
  assert_eq!(cores[1].recv(Taurus::COL), 3);
  assert_eq!(cores[3].recv(Taurus::COL), 3);
}
// ------------------------------------------------------------

#[test]
fn get_submatrices_dim_along_axis_more_processors() {
  let submatrices_dims = get_submatrices_dim_along_axis(6, 4);
  assert_eq!(submatrices_dims, vec![1,1,1,1,0,0]);
}

#[test]
fn get_submatrices_dim_along_axis_equal_size() {
  let submatrices_dims =get_submatrices_dim_along_axis(4, 4);
  assert_eq!(submatrices_dims, vec![1,1,1,1]);
}

#[test]
fn get_submatrices_dim_along_axis_less_processors() {
  let submatrices_dims = get_submatrices_dim_along_axis(6, 17);
  assert_eq!(submatrices_dims, vec![3,3,3,3,3,2]);
}

#[test]
fn get_submatrices_dim_along_axis_less_processors_divisible(){
  let submatrices_dims = get_submatrices_dim_along_axis(6, 18);
  assert_eq!(submatrices_dims, vec![3,3,3,3,3,3]);
}

// ------------------------------------------------------------

#[test]
fn get_submatrices_dim_square_equal(){
  let submatrices_dims = get_submatrices_dim(2,2,2,2);
  assert_eq!(submatrices_dims[0],
    SubmatrixDim {
      start_row : 0,
      start_col : 0,
      width : 1,
      height : 1,
  });
  assert_eq!(submatrices_dims[1],
    SubmatrixDim {
      start_row : 0,
      start_col : 1,
      width : 1,
      height : 1,
  });
  assert_eq!(submatrices_dims[2],
    SubmatrixDim {
      start_row : 1,
      start_col : 0,
      width : 1,
      height : 1,
  });
  assert_eq!(submatrices_dims[3],
    SubmatrixDim {
      start_row : 1,
      start_col : 1,
      width : 1,
      height : 1,
  });
}

#[test]
fn get_submatrices_dim_square_diff(){
  let submatrices_dims = get_submatrices_dim(2,2,3,3);
  assert_eq!(submatrices_dims[0],
    SubmatrixDim {
      start_row : 0,
      start_col : 0,
      width : 2,
      height : 2,
  });
  assert_eq!(submatrices_dims[1],
    SubmatrixDim {
      start_row : 0,
      start_col : 2,
      width : 1,
      height : 2,
  });
  assert_eq!(submatrices_dims[2],
    SubmatrixDim {
      start_row : 2,
      start_col : 0,
      width : 2,
      height : 1,
  });
  assert_eq!(submatrices_dims[3],
    SubmatrixDim {
      start_row : 2,
      start_col : 2,
      width : 1,
      height : 1,
  });
}

// ------------------------------------------------------------

#[test]
fn get_matrix_slices_equal_dim(){
  let m = vec![
    vec![1,2,3,4],
    vec![5,6,7,8],
    vec![9,10,11,12],
    vec![13,14,15,16]
  ];
  let dims = vec![
    SubmatrixDim {
      start_row : 0,
      start_col : 0,
      width : 2,
      height : 2,
    },
    SubmatrixDim {
      start_row : 0,
      start_col : 2,
      width : 2,
      height : 2,
    },
    SubmatrixDim {
      start_row : 2,
      start_col : 0,
      width : 2,
      height : 2,
    },
    SubmatrixDim {
      start_row : 2,
      start_col : 2,
      width : 2,
      height : 2,
    },
  ];
  let submatrices_dims = get_matrix_slices(&m, &dims);
  assert_eq!(submatrices_dims, 
    vec![
      vec![
        vec![1,2],
        vec![5,6]
      ],
      vec![
        vec![3,4],
        vec![7,8]
      ],
      vec![
        vec![9,10],
        vec![13,14]
      ],
      vec![
        vec![11,12],
        vec![15,16]
      ]
    ]);
}

#[test]
fn get_matrix_slices_diff_dim(){
  let m = vec![
    vec![1,2,3,4],
    vec![5,6,7,8],
    vec![9,10,11,12],
    vec![13,14,15,16]
  ];
  let dims = vec![
    SubmatrixDim {
      start_row : 0,
      start_col : 0,
      width : 3,
      height : 3,
    },
    SubmatrixDim {
      start_row : 0,
      start_col : 3,
      width : 1,
      height : 3,
    },
    SubmatrixDim {
      start_row : 3,
      start_col : 0,
      width : 3,
      height : 1,
    },
    SubmatrixDim {
      start_row : 3,
      start_col : 3,
      width : 1,
      height : 1,
    },
  ];
  let submatrices_dims = get_matrix_slices(&m, &dims);
  assert_eq!(submatrices_dims, 
    vec![
      vec![
        vec![1,2,3],
        vec![5,6,7],
        vec![9,10,11]
      ],
      vec![
        vec![4],
        vec![8],
        vec![12]
      ],
      vec![
        vec![13,14,15]
      ],
      vec![
        vec![16]
      ]
    ]);
}

// ------------------------------------------------------------

#[test]
fn get_submatrices_square_equal(){
  let m = vec![
    vec![1,2],
    vec![3,4]
  ];
  let submatrices = get_submatrices(2, 2, &m);
  assert_eq!(submatrices, 
    vec![
      vec![
        vec![1]
      ],
      vec![
        vec![2]
      ],
      vec![
        vec![3]
      ],
      vec![
        vec![4]
      ]
    ]);
}

#[test]
fn get_submatrices_square_diff(){
  let m = vec![
    vec![1,2,3],
    vec![4,5,6],
    vec![7,8,9]
  ];
  let submatrices = get_submatrices(2,2,&m);
  assert_eq!(submatrices, 
    vec![
      vec![
        vec![1,2],
        vec![4,5]
      ],
      vec![
        vec![3],
        vec![6]
      ],
      vec![
        vec![7,8]
      ],
      vec![
        vec![9]
      ]
    ]);
}

// ------------------------------------------------------------

#[test]
fn test_core_debug_time_progresses(){
  let mut processor : Processor<(),i32, TaurusCoreInfo<i32>> = 
    Processor::new(2,2, Box::new(TaurusNetworkBuilder{}));
  let mut cores = processor.build_network();
  // Check that horizontal broadcast works
  
  let p3 = move |core_info: &mut TaurusCoreInfo<i32>, debugger : &mut CoreDebugger| {
    dbg!(&debugger.get_curr_elapsed().as_millis());
    let t = Instant::now();
    while t.elapsed() < Duration::new(0,500000000) {
      continue
    }
    dbg!(&debugger.get_curr_elapsed().as_millis());
    core_info.debug_send(Taurus::LEFT,1, &mut Some(debugger));
  };

  let p2 = move |core_info: &mut TaurusCoreInfo<i32>, debugger : &mut CoreDebugger| {
    dbg!(&debugger.get_curr_elapsed().as_millis());
    core_info.debug_recv(Taurus::RIGHT, &mut Some(debugger));
    dbg!(&debugger.get_curr_elapsed().as_millis());
  };

  processor.run_debug_core(p3, cores.pop().unwrap());
  processor.run_debug_core(p2, cores.pop().unwrap());
  
  processor.collect_results();
  
  dbg!(&processor.debugs[0].elapsed.as_millis());
  dbg!(&processor.debugs[1].elapsed.as_millis());

  assert!(processor.debugs[0].elapsed.as_millis() > 400);
  assert!(processor.debugs[1].elapsed.as_millis() > 400);
}

#[test]
fn test_core_debug_time_handles_sleep(){
  let mut processor : Processor<(),i32, TaurusCoreInfo<i32>> = 
    Processor::new(2,2, Box::new(TaurusNetworkBuilder{}));
  let mut cores = processor.build_network();
  // Check that horizontal broadcast works
  
  let p3 = move |core_info: &mut TaurusCoreInfo<i32>, debugger : &mut CoreDebugger| {
    dbg!(&debugger.get_curr_elapsed().as_millis());
    sleep(Duration::new(2, 0));

    dbg!(&debugger.get_curr_elapsed().as_millis());
    core_info.debug_send(Taurus::LEFT,1, &mut Some(debugger));
    dbg!(&debugger.get_curr_elapsed().as_millis());
  };

  let p2 = move |core_info: &mut TaurusCoreInfo<i32>, debugger : &mut CoreDebugger| {
    core_info.debug_recv(Taurus::RIGHT, &mut Some(debugger));
  };

  processor.run_debug_core(p3, cores.pop().unwrap());
  processor.run_debug_core(p2, cores.pop().unwrap());
  
  processor.collect_results();
  
  dbg!(&processor.debugs[0].elapsed.as_millis());
  dbg!(&processor.debugs[1].elapsed.as_millis());

  assert!(processor.debugs[0].elapsed.as_millis() < 10);
  assert!(processor.debugs[1].elapsed.as_millis() < 10);
}

#[test]
fn test_core_debug_time_received_is_less(){
  let mut processor : Processor<(),i32, TaurusCoreInfo<i32>> = 
    Processor::new(2,2, Box::new(TaurusNetworkBuilder{}));
  let mut cores = processor.build_network();
  // Check that horizontal broadcast works
  
  let p3 = move |core_info: &mut TaurusCoreInfo<i32>, debugger : &mut CoreDebugger| {
    dbg!(&debugger.get_curr_elapsed().as_millis());
    core_info.debug_send(Taurus::LEFT,1, &mut Some(debugger));
    dbg!(&debugger.get_curr_elapsed().as_millis());
  };

  let p2 = move |core_info: &mut TaurusCoreInfo<i32>, debugger : &mut CoreDebugger| {
    let t = Instant::now();
    while t.elapsed() < Duration::new(0,500000000) {
      continue
    }
    dbg!(&debugger.get_curr_elapsed().as_millis());
    core_info.debug_recv(Taurus::RIGHT, &mut Some(debugger));
    dbg!(&debugger.get_curr_elapsed().as_millis());
  };

  processor.run_debug_core(p3, cores.pop().unwrap());
  processor.run_debug_core(p2, cores.pop().unwrap());
  
  processor.collect_results();
  
  dbg!(&processor.debugs[0].elapsed.as_millis());
  dbg!(&processor.debugs[0].row);
  dbg!(&processor.debugs[0].col);
  dbg!(&processor.debugs[1].elapsed.as_millis());

  assert!(processor.debugs[1].elapsed.as_millis() < 10);
  assert!(processor.debugs[0].elapsed.as_millis() > 450);
}
