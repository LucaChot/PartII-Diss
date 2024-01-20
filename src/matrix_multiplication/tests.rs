use super::*;
use std::collections::VecDeque;
use cannons; 


#[test]
fn test_serial_matrix_multiplication_square(){
  let matrix_a : Matrix<isize> = vec![
    vec![1,2],
    vec![3,4]
  ];
  
  let matrix_b : Matrix<isize> = vec![
    vec![1,2],
    vec![3,4]
  ];

  let matrix_c : Matrix<isize> = vec![
    vec![0,0],
    vec![0,0]
  ];

  let result = serial_matrix_multiplication(&matrix_a, &matrix_b, &matrix_c, singleton_matrix_multiplication);

  assert_eq!(result, vec![
    vec![7,10],
    vec![15,22]]);
}

#[test]
fn test_serial_matrix_multiplication_non_square(){
  let matrix_a : Matrix<isize> = vec![
    vec![1,2],
    vec![3,4],
    vec![5,6],
  ];
  
  let matrix_b : Matrix<isize> = vec![
    vec![1,2],
    vec![3,4]
  ];

  let matrix_c : Matrix<isize> = vec![
    vec![0,0],
    vec![0,0],
    vec![0,0]
  ];

  let result = serial_matrix_multiplication(&matrix_a, &matrix_b, &matrix_c, singleton_matrix_multiplication);

  assert_eq!(result, vec![
    vec![7,10],
    vec![15,22],
    vec![23,34]]);
}

#[test]
fn test_cannon_shift_a(){
  let matrix_a : VecDeque<Matrix<isize>> = VecDeque::from(vec![
    vec![ vec![0] ], vec![ vec![1] ], vec![ vec![2] ],
    vec![ vec![3] ], vec![ vec![4] ], vec![ vec![5] ],
    vec![ vec![6] ], vec![ vec![7] ], vec![ vec![8] ],
  ]);

  let correct = vec![
    vec![ vec![0] ], vec![ vec![1] ], vec![ vec![2] ],
    vec![ vec![4] ], vec![ vec![5] ], vec![ vec![3] ],
    vec![ vec![8] ], vec![ vec![6] ], vec![ vec![7] ],
  ];

  let result = cannons::cannon_setup_a(matrix_a, (3, 3));

  for (res, correct) in result.iter().zip(correct.iter()) {
    assert_eq!(res, correct);
  }
}

#[test]
fn test_cannon_shift_b(){
  let matrix_a : VecDeque<Matrix<isize>> = VecDeque::from(vec![
    vec![ vec![0] ], vec![ vec![1] ], vec![ vec![2] ],
    vec![ vec![3] ], vec![ vec![4] ], vec![ vec![5] ],
    vec![ vec![6] ], vec![ vec![7] ], vec![ vec![8] ],
  ]);

  let correct = vec![
    vec![ vec![0] ], vec![ vec![4] ], vec![ vec![8] ],
    vec![ vec![3] ], vec![ vec![7] ], vec![ vec![2] ],
    vec![ vec![6] ], vec![ vec![1] ], vec![ vec![5] ],
  ];

  let result = cannons::cannon_setup_b(matrix_a, (3, 3));

  for (res, correct) in result.iter().zip(correct.iter()) {
    assert_eq!(res, correct);
  }
}
