use std::fmt::{Debug,Display,Formatter,Result};
use std::time::Duration;
use crate::Sendable;
use crate::Multiplicable;

#[derive(Clone,Debug)]
pub struct Msg {
  w : isize,
  p : usize,
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.w, self.p)
    }
}

impl Msg {
  pub fn new(w : isize, p : usize) -> Msg {
    return Msg {
      w,
      p,
    }
  }

  pub fn zip(matrix_w : Matrix<isize>, matrix_p : Matrix<usize>) -> Matrix<Msg> {
  matrix_w.into_iter().zip(matrix_p.into_iter())
    .map(|(w_row, p_row)| w_row.into_iter().zip(p_row.into_iter())
      .map(|(w, p)| Msg { w, p }  ).collect::<Vec<Msg>>()
    ).collect::<Matrix<Msg>>()
  }

  pub fn unzip(matrix_m : Matrix<Msg>) -> (Matrix<isize>, Matrix<usize>) {
  matrix_m.into_iter().fold(
    (Vec::new(), Vec::new()), |(mut matrix_w, mut matrix_p), inner| {
      let (w_inner, p_inner): (Vec<isize>, Vec<usize>) = inner.into_iter()
        .fold((Vec::new(), Vec::new()), |(mut vec_w, mut vec_p), msg| {
          vec_w.push(msg.w);
          vec_p.push(msg.p);
          (vec_w, vec_p)
        });
      matrix_w.push(w_inner);
      matrix_p.push(p_inner);
      (matrix_w, matrix_p)
    })
  }

  pub fn get_w(&self) -> isize {
    return self.w;
  }

  pub fn get_p(&self) -> usize {
    return self.p;
  }
}

impl Sendable for Msg {}
impl Multiplicable for Msg {
  fn neutral_element (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|i| Msg{ w : -1, p : i }).collect()).collect()
  }
  fn singleton_matrix<T : Multiplicable>(a : Self, b : Self, c : Self) -> Self {
    let mut temp = c;
    if a.w != -1 && b.w != -1 && ( temp.w == -1 || a.w + b.w < temp.w ){
      temp.w = a.w + b.w;
      temp.p = b.p;
    }
    temp
  }
}

impl Sendable for isize {}
impl Multiplicable for isize {
  fn neutral_element (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect()
  }
  fn singleton_matrix<T : Multiplicable>(a : Self, b : Self, c : Self) -> Self {
    c + a * b
  }
}

impl Sendable for usize {}
impl Multiplicable for usize {
  fn neutral_element (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect()
  }
  fn singleton_matrix<T : Multiplicable>(a : Self, b : Self, c : Self) -> Self {
    c + a * b
  }
}

impl Sendable for f64 {}
impl Multiplicable for f64 {
  fn neutral_element (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|_| 0.0).collect()).collect()
  }
  fn singleton_matrix<T : Multiplicable>(a : Self, b : Self, c : Self) -> Self {
    c + a * b
  }
}

impl<T:Sendable> Sendable for Vec<Vec<T>> {}
pub type Matrix<T> = Vec<Vec<T>>;

impl Sendable for Duration {}

impl<X:Sendable, Y:Sendable> Sendable for (X,Y) {}
impl<X:Sendable, Y:Sendable, Z:Sendable> Sendable for (X,Y,Z) {}
