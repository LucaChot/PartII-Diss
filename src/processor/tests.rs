use super::*;
use super::taurus::*;


#[test]
fn general_correct_length(){
  let network_builder = TaurusNetworkBuilder;
  let processor : Processor <i32,i32, TaurusCore<i32>> = 
    Processor::new(2,2, network_builder);
  assert_eq!(processor.cores.len(), 4);
}

#[test]
fn general_correct_connection(){
  let network_builder = TaurusNetworkBuilder;
  let mut processor : Processor <i32,i32, TaurusCore<i32>> = 
    Processor::new(2,2, network_builder);
  // Check that horizontal broadcast works
  processor.cores[0].send(1, &TaurusOption::UP);
  assert_eq!(processor.cores[2].recv(&TaurusOption::DOWN), 1);

  processor.cores[0].send(2, &TaurusOption::RIGHT);
  assert_eq!(processor.cores[1].recv(&TaurusOption::LEFT), 2);

  processor.cores[0].send(3, &TaurusOption::DOWN);
  assert_eq!(processor.cores[2].recv(&TaurusOption::UP), 3);

  processor.cores[0].send(4, &TaurusOption::LEFT);
  assert_eq!(processor.cores[1].recv(&TaurusOption::RIGHT), 4);

  processor.cores[3].send(1, &TaurusOption::UP);
  assert_eq!(processor.cores[1].recv(&TaurusOption::DOWN), 1);

  processor.cores[3].send(2, &TaurusOption::RIGHT);
  assert_eq!(processor.cores[2].recv(&TaurusOption::LEFT), 2);

  // Check that vertical broadcast works
  processor.cores[3].send(3, &TaurusOption::DOWN);
  assert_eq!(processor.cores[1].recv(&TaurusOption::UP), 3);

  processor.cores[3].send(4, &TaurusOption::LEFT);
  assert_eq!(processor.cores[2].recv(&TaurusOption::RIGHT), 4);
}

#[test]
fn general_correct_broadcast(){
  let network_builder = TaurusNetworkBuilder;
  let mut processor : Processor <i32,i32, TaurusCore<i32>> = 
    Processor::new(2,2, network_builder);

  // Check that horizontal broadcast works
  processor.cores[0].send(0, &TaurusOption::ROW);
  assert_eq!(processor.cores[0].recv(&TaurusOption::ROW), 0);
  assert_eq!(processor.cores[1].recv(&TaurusOption::ROW), 0);

  processor.cores[3].send(1, &TaurusOption::ROW);
  assert_eq!(processor.cores[2].recv(&TaurusOption::ROW), 1);
  assert_eq!(processor.cores[3].recv(&TaurusOption::ROW), 1);

  // Check that vertical broadcast works
  processor.cores[2].send(2, &TaurusOption::COL);
  assert_eq!(processor.cores[0].recv(&TaurusOption::COL), 2);
  assert_eq!(processor.cores[2].recv(&TaurusOption::COL), 2);

  processor.cores[1].send(3, &TaurusOption::COL);
  assert_eq!(processor.cores[1].recv(&TaurusOption::COL), 3);
  assert_eq!(processor.cores[3].recv(&TaurusOption::COL), 3);
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

