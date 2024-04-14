use super::*;
use std::thread::sleep;

#[test]
fn general_correct_length(){
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let bchannels = processor.create_taurus();
  assert_eq!(bchannels.len(), 4);
}

#[test]
fn general_correct_connection(){
  let processor : Processor<i32, i32> = Processor::new(2,2);
  let mut cores = processor.create_taurus();
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
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let mut cores = processor.create_taurus();

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
  let submatrices_dims = Processor::<i32,i32>::get_submatrices_dim_along_axis(6, 4);
  assert_eq!(submatrices_dims, vec![1,1,1,1,0,0]);
}

#[test]
fn get_submatrices_dim_along_axis_equal_size() {
  let submatrices_dims = Processor::<i32,i32>::get_submatrices_dim_along_axis(4, 4);
  assert_eq!(submatrices_dims, vec![1,1,1,1]);
}

#[test]
fn get_submatrices_dim_along_axis_less_processors() {
  let submatrices_dims = Processor::<i32,i32>::get_submatrices_dim_along_axis(6, 17);
  assert_eq!(submatrices_dims, vec![3,3,3,3,3,2]);
}

#[test]
fn get_submatrices_dim_along_axis_less_processors_divisible(){
  let submatrices_dims = Processor::<i32,i32>::get_submatrices_dim_along_axis(6, 18);
  assert_eq!(submatrices_dims, vec![3,3,3,3,3,3]);
}

// ------------------------------------------------------------

#[test]
fn get_submatrices_dim_square_equal(){
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let submatrices_dims = processor.get_submatrices_dim(2,2);
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
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let submatrices_dims = processor.get_submatrices_dim(3,3);
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
  let submatrices_dims = Processor::<i32,i32>::get_matrix_slices(&m, &dims);
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
  let submatrices_dims = Processor::<i32,i32>::get_matrix_slices(&m, &dims);
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
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let submatrices = processor.get_submatrices(&m);
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
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let submatrices = processor.get_submatrices(&m);
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
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let mut cores = processor.create_taurus();
  // Check that horizontal broadcast works
  
  dbg!(cores[2].core_debug.get_curr_elapsed());
  cores[0].send(Taurus::UP,1);

  sleep(Duration::new(2, 0));
  assert_eq!(cores[2].recv(Taurus::DOWN), 1);
  assert!(cores[2].core_debug.get_curr_elapsed().as_millis() >=  2000);
}

#[test]
fn test_core_debug_time_received_is_less(){
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let mut cores = processor.create_taurus();
  // Check that horizontal broadcast works
  
  let true_elapsed = cores[2].core_debug.get_curr_elapsed();
  cores[2].core_debug.update_elapsed(true_elapsed + Duration::new(2,0));
  cores[0].send(Taurus::UP,1);
  assert!(cores[2].core_debug.get_curr_elapsed().as_millis() >= 2000);
  assert_eq!(cores[2].recv(Taurus::DOWN), 1);
  assert!(cores[2].core_debug.get_curr_elapsed().as_millis() >= 2000);
}

#[test]
fn test_core_debug_time_received_is_greater(){
  let processor : Processor<i32,i32> = Processor::new(2,2);
  let mut cores = processor.create_taurus();
  // Check that horizontal broadcast works
  
  let true_elapsed = cores[0].core_debug.get_curr_elapsed();
  cores[0].core_debug.update_elapsed(true_elapsed + Duration::new(2,0));
  dbg!(&cores[0].core_debug.get_curr_elapsed().as_nanos());

  cores[0].send(Taurus::COL,1);
  
  assert!(cores[2].core_debug.get_curr_elapsed().as_millis() < 1000);
  dbg!(&cores[2].core_debug.get_curr_elapsed().as_nanos());
  assert_eq!(cores[2].recv(Taurus::COL), 1);
  dbg!(&cores[2].core_debug.get_curr_elapsed().as_nanos());
  assert!(cores[2].core_debug.get_curr_elapsed().as_millis() >= 2000);
}

