use super::*;

#[test]
fn general_correct_length(){

  let bchannels = general_processor::<i32>((2, 2));
  assert_eq!(bchannels.len(), 4);
}

#[test]
fn general_correct_connection(){
  let cores = general_processor::<i32>((2, 2));
  // Check that horizontal broadcast works
  cores[0].core_comm.up.send(1);
  assert_eq!(cores[2].core_comm.down.recv(), 1);

  cores[0].core_comm.right.send(2);
  assert_eq!(cores[1].core_comm.left.recv(), 2);

  // Check that vertical broadcast works
  cores[0].core_comm.down.send(3);
  assert_eq!(cores[2].core_comm.up.recv(), 3);

  cores[0].core_comm.left.send(4);
  assert_eq!(cores[1].core_comm.right.recv(), 4);

  cores[3].core_comm.up.send(1);
  assert_eq!(cores[1].core_comm.down.recv(), 1);

  cores[3].core_comm.right.send(2);
  assert_eq!(cores[2].core_comm.left.recv(), 2);

  // Check that vertical broadcast works
  cores[3].core_comm.down.send(3);
  assert_eq!(cores[1].core_comm.up.recv(), 3);

  cores[3].core_comm.left.send(4);
  assert_eq!(cores[2].core_comm.right.recv(), 4);
}

#[test]
fn general_correct_broadcast(){
  let cores = general_processor::<i32>((2, 2));

  // Check that horizontal broadcast works
  cores[0].core_comm.row.send(0);
  assert_eq!(cores[0].core_comm.row.recv(), 0);
  assert_eq!(cores[1].core_comm.row.recv(), 0);

  cores[3].core_comm.row.send(1);
  assert_eq!(cores[2].core_comm.row.recv(), 1);
  assert_eq!(cores[3].core_comm.row.recv(), 1);

  // Check that vertical broadcast works
  cores[2].core_comm.col.send(2);
  assert_eq!(cores[0].core_comm.col.recv(), 2);
  assert_eq!(cores[2].core_comm.col.recv(), 2);

  cores[1].core_comm.col.send(3);
  assert_eq!(cores[1].core_comm.col.recv(), 3);
  assert_eq!(cores[3].core_comm.col.recv(), 3);
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
