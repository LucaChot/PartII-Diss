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
