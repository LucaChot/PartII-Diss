use crate::broadcast::{BChannel, Sendable, Channel};

pub struct CoreComm<T:Sendable>{
  pub left : Channel<T>,
  pub right : Channel<T>,
  pub up : Channel<T>,
  pub down : Channel<T>,
  pub row : BChannel<T>,
  pub col : BChannel<T>,
}

impl<T : Sendable> CoreComm<T> {
  fn new() -> CoreComm<T> {
    CoreComm { 
      left: Channel::empty(),
      right: Channel::empty(),
      up: Channel::empty(),
      down: Channel::empty(),
      row: BChannel::empty(),
      col: BChannel::empty()
    }
  } 
}

pub struct CoreInfo<T : Sendable> {
  pub row : usize,
  pub col : usize,
  pub core_comm : CoreComm<T>,
} 

// TODO : Update comment
/// This function returns a 2D Vec of BChannel<T> representing the broadcast 
/// channels for a hashtag processor (only vertical and horizontal broadcasts)
///
/// # Arguemnts
/// * `rows` - Number of rows in the processor matrix
/// * `cols` - Number of columns in the processor matrix
///
/// # Returns
/// Returns Vec<Vec<BCHannel<T>>> which has out length equal to the number of 
/// cores in the processor and inner length equal to the number of channels 
/// accessible by the corresponding core. 
pub fn general_processor<T: Sendable>((rows, cols) : (usize,usize)) -> Vec<CoreInfo<T>> {
  let num_cores = cols * rows;
  let mut cores : Vec<CoreInfo<T>> = Vec::with_capacity(num_cores);
  for row in 0..rows {
    for col in 0..cols {
      cores.push(CoreInfo{ row, col, core_comm : CoreComm::new() });
    }
  }

  for i in 0..rows {
    let mut bchannels : Vec<BChannel<T>> = BChannel::new(cols);
    for step in 0..cols {
      cores[rows * i + step].core_comm.row = bchannels.pop().unwrap();
    }
  }

  for i in 0..cols {
    let mut bchannels : Vec<BChannel<T>> = BChannel::new(cols);
    for step in 0..rows {
      cores[rows * step + i].core_comm.col = bchannels.pop().unwrap();
    }
  }
  
  for i in 0..num_cores {
    let (up, down) = Channel::new();
    cores[i].core_comm.up = up;
    cores[( num_cores + i - cols ) % num_cores].core_comm.down = down; 

    let (right, left) = Channel::new();
    cores[i].core_comm.right = right;
    cores[i - ( i % cols ) + ( (i +  1) % cols )].core_comm.left = left; 
  }
  
  return cores
}

/// This function returns a Vec containing the dimensions of the submatrices to 
/// be assigned to each processor given the length of the array of processors 
/// and the matrix along a given axis
///
/// # Arguemnts
/// * `processor_length` - Length of processor along a given axis
/// * `matrix_length` - Length of matrix along a given axis
///
/// # Returns
/// Returns the Vec<usize> of length `processor_length` which contains the 
/// length along the axis of the submatrix to be assigned to each processor
fn get_submatrices_dim_along_axis(processor_length : usize, matrix_length : usize) -> Vec<usize> {
  let min_len : usize = matrix_length / processor_length;
  let remaining : usize = matrix_length - ( processor_length * min_len );
  let mut submatrix_dimensions : Vec<usize> = vec![min_len; processor_length]; 

  for element in submatrix_dimensions[0..remaining].iter_mut() {
    *element += 1;
  }

  submatrix_dimensions
}

#[derive(Copy,Clone,Debug, PartialEq)]
pub struct SubmatrixDim {
  pub start_row : usize,
  pub start_col : usize,
  pub width : usize,
  pub height : usize,
}

pub fn get_submatrices_dim(processor_dim : (usize,usize), matrix_dim : (usize,usize)) -> Vec<SubmatrixDim> {
  let dim_along_y = get_submatrices_dim_along_axis(processor_dim.0, matrix_dim.0);
  let dim_along_x = get_submatrices_dim_along_axis(processor_dim.1, matrix_dim.1);

  dim_along_y.iter().fold((0, Vec::new()), |(start_row, mut result), &height| {
    dim_along_x.iter().fold(0, |start_col, &width| {
      result.push(SubmatrixDim {
        start_row,
        start_col,
        width,
        height,
      });
      start_col + width
    });
    (start_row + height, result)
    }).1
}

fn get_matrix_slices<T:Clone>(matrix : &Vec<Vec<T>>, dims : &Vec<SubmatrixDim>) -> Vec<Vec<Vec<T>>> {
  dims.iter().map(|&dim| 
    matrix.iter().skip(dim.start_row).take(dim.height)
       .map(|row| row.iter().skip(dim.start_col).take(dim.width).cloned().collect::<Vec<_>>())
       .collect::<Vec<_>>()
  ).collect::<Vec<_>>()
}

pub fn get_submatrices<T: Clone>(matrix : &Vec<Vec<T>>, processor_dim : (usize,usize)) -> Vec<Vec<Vec<T>>> {
  let matrix_rows = matrix.len();
  let matrix_cols = matrix[0].len();

  let submatrices_dim = get_submatrices_dim(processor_dim, (matrix_rows, matrix_cols));
  
  get_matrix_slices(matrix, &submatrices_dim)
}


#[cfg(test)]
mod tests;
