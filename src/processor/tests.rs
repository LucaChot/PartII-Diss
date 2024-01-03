use crate::processor::fox_otto_processor;

use super::*;

// ------------------------------------------------------------
#[test]
fn hashtag_correct_length(){

  let bchannels = hashtag_processor::<i32>(2, 2);
  assert_eq!(bchannels.len(), 4);
}

#[test]
fn hashtag_correct_connection(){
  let bchannels = hashtag_processor::<i32>(2, 2);
  // Check that horizontal broadcast works
  bchannels[0][0].send(1);
  assert_eq!(bchannels[0][0].recv().unwrap(), 1);
  assert_eq!(bchannels[1][0].recv().unwrap(), 1);

  bchannels[2][0].send(2);
  assert_eq!(bchannels[2][0].recv().unwrap(), 2);
  assert_eq!(bchannels[3][0].recv().unwrap(), 2);

  // Check that vertical broadcast works
  bchannels[0][1].send(3);
  assert_eq!(bchannels[0][1].recv().unwrap(), 3);
  assert_eq!(bchannels[2][1].recv().unwrap(), 3);

  bchannels[1][1].send(4);
  assert_eq!(bchannels[1][1].recv().unwrap(), 4);
  assert_eq!(bchannels[3][1].recv().unwrap(), 4);
  
}

// ------------------------------------------------------------

#[test]
fn fox_otto_correct_length(){

  let bchannels = fox_otto_processor::<i32>(2, 2);
  assert_eq!(bchannels.len(), 4);
}

#[test]
fn fox_otto_correct_connection(){
  let bchannels = fox_otto_processor::<i32>(2, 2);
  // Check that horizontal broadcast works
  let _ = bchannels[0].1.send(0);
  assert_eq!(bchannels[2].2.recv().unwrap(), 0);

  let _ = bchannels[1].1.send(1);
  assert_eq!(bchannels[3].2.recv().unwrap(), 1);

  let _ = bchannels[2].1.send(2);
  assert_eq!(bchannels[0].2.recv().unwrap(), 2);

  let _ = bchannels[3].1.send(3);
  assert_eq!(bchannels[1].2.recv().unwrap(), 3);
}

#[test]
fn fox_otto_correct_broadcast(){
  let bchannels = fox_otto_processor::<i32>(2, 2);

  // Check that horizontal broadcast works
  bchannels[0].0.send(0);
  assert_eq!(bchannels[0].0.recv().unwrap(), 0);
  assert_eq!(bchannels[1].0.recv().unwrap(), 0);

  bchannels[2].0.send(1);
  assert_eq!(bchannels[2].0.recv().unwrap(), 1);
  assert_eq!(bchannels[3].0.recv().unwrap(), 1);
}

// ------------------------------------------------------------

#[test]
fn get_submatrices_dim_along_axis_more_processors() {
  let submatrices_dims = get_submatrices_dim_along_axis(6, 4);
  assert_eq!(submatrices_dims, vec![1,1,1,1,0,0]);
}

#[test]
fn get_submatrices_dim_along_axis_equal_size() {
  let submatrices_dims = get_submatrices_dim_along_axis(4, 4);
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
  let submatrices_dims = get_submatrices_dim((2,2), (2,2));
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
  let submatrices_dims = get_submatrices_dim((2,2), (3,3));
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
  let dims : (usize, usize) = (2,2);
  let m = vec![
    vec![1,2],
    vec![3,4]
  ];
  let submatrices = get_submatrices(&m, dims);
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
  let dims : (usize, usize) = (2,2);
  let m = vec![
    vec![1,2,3],
    vec![4,5,6],
    vec![7,8,9]
  ];
  let submatrices = get_submatrices(&m, dims);
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
