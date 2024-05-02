use std::collections::VecDeque;

use crate::processor::get_submatrices;

use super::*;

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

  let matrix_c : Matrix<isize> = isize::initial_c(&matrix_a, &matrix_b);
  let result = serial_matmul(&matrix_a, &matrix_b, &matrix_c);

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

  let matrix_c : Matrix<isize> = isize::initial_c(&matrix_a, &matrix_b);

  let result = serial_matmul(&matrix_a, &matrix_b, &matrix_c);

  assert_eq!(result, vec![
    vec![7,10],
    vec![15,22],
    vec![23,34]]);
}

#[test]
fn test_cannon_shift_a(){
  let matrix_a : Matrix<isize> = vec![
    vec![0,1,2],
    vec![3,4,5],
    vec![6,7,8]
  ];

  let correct = vec![
    vec![ vec![0] ], vec![ vec![1] ], vec![ vec![2] ],
    vec![ vec![4] ], vec![ vec![5] ], vec![ vec![3] ],
    vec![ vec![8] ], vec![ vec![6] ], vec![ vec![7] ],
  ];

  let result = {
    let rows = 3;
    let cols = 3;
    let submatrices_a = VecDeque::from(get_submatrices(rows, cols, &matrix_a));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| row * cols +((cols + col - row) % cols))
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<isize>>>();
    submatrices_a.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    result
  };

  for (res, correct) in result.iter().zip(correct.iter()) {
    assert_eq!(res, correct);
  }
}

#[test]
fn test_cannon_shift_b(){
  let matrix_b : Matrix<isize> = vec![
    vec![0,1,2],
    vec![3,4,5],
    vec![6,7,8]
  ];

  let correct = vec![
    vec![ vec![0] ], vec![ vec![4] ], vec![ vec![8] ],
    vec![ vec![3] ], vec![ vec![7] ], vec![ vec![2] ],
    vec![ vec![6] ], vec![ vec![1] ], vec![ vec![5] ],
  ];

  let result = {
    let rows = 3;
    let cols = 3;
    let submatrices_b = VecDeque::from(get_submatrices(rows, cols, &matrix_b));
    let indices : Vec<usize> = (0..rows)
      .flat_map(|row| (0..cols)
                .map(|col| ((rows + row - col) % rows) * cols + col)
                .collect::<Vec<_>>())
      .collect();
    let mut result = indices.iter().map(|_| Vec::new()).collect::<VecDeque<Matrix<isize>>>();
    submatrices_b.into_iter().zip(indices.iter()).map(|(m, &index)| result[index] = m).count();

    result
  };

  for (res, correct) in result.iter().zip(correct.iter()) {
    assert_eq!(res, correct);
  }
}
