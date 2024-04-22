use std::fmt::{Debug,Display,Formatter,Result};
use std::time::Duration;
use crate::broadcast::Sendable;
use crate::matmul::Multiplicable;
use serde::{Serialize,Deserialize};


#[derive(Clone,Debug,Serialize, Deserialize)]
pub struct Msg {
  w : f64,
  p : usize,
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.w, self.p)
    }
}

impl Msg {
  pub fn new(w : f64, p : usize) -> Msg {
    return Msg {
      w,
      p,
    }
  }

  pub fn zip(matrix_w : &Matrix<f64>, matrix_p : &Matrix<usize>) -> Matrix<Msg> {
  matrix_w.iter().zip(matrix_p.iter())
    .map(|(w_row, p_row)| w_row.iter().zip(p_row.into_iter())
      .map(|(&w, &p)| Msg { w, p }  ).collect::<Vec<Msg>>()
    ).collect::<Matrix<Msg>>()
  }

  pub fn unzip(matrix_m : &Matrix<Msg>) -> (Matrix<f64>, Matrix<usize>) {
  matrix_m.iter().fold(
    (Vec::new(), Vec::new()), |(mut matrix_w, mut matrix_p), inner| {
      let (w_inner, p_inner): (Vec<f64>, Vec<usize>) = inner.iter()
        .fold((Vec::new(), Vec::new()), |(mut vec_w, mut vec_p), msg| {
          vec_w.push(msg.get_w());
          vec_p.push(msg.get_p());
          (vec_w, vec_p)
        });
      matrix_w.push(w_inner);
      matrix_p.push(p_inner);
      (matrix_w, matrix_p)
    })
  }

  pub fn get_w(&self) -> f64 {
    return self.w;
  }

  pub fn get_p(&self) -> usize {
    return self.p;
  }
}

impl Sendable for Msg {}
impl Multiplicable for Msg {
  fn initial_c (a : &Matrix<Self>, _ : &Matrix<Self>) -> Matrix<Self> {
    a.clone()
  }
  fn singleton_matrix(a : Self, b : Self, c : Self) -> Self {
    let mut temp = c;
    if a.w > 0.0 && b.w > 0.0  && ( temp.w < 0.0 || a.w + b.w < temp.w ){
      temp.w = a.w + b.w;
      temp.p = b.p;
    }
    temp
  }

  fn neutral_matrix (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|j| (0..cols).map(|i| Msg{ 
      w : if i == j {0.0} else {-1.0}, 
      p : i 
    }).collect()).collect()
  }
}

impl Sendable for () {}

impl Sendable for isize {}
impl Multiplicable for isize {
  fn initial_c (a : &Matrix<Self>, b : &Matrix<Self>) -> Matrix<Self> {
    a.iter().map(|_| b[0].iter().map(|_| 0).collect()).collect()
  }
  fn singleton_matrix(a : Self, b : Self, c : Self) -> Self {
    c + a * b
  }
  fn neutral_matrix (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect()
  }
}

impl Sendable for usize {}
impl Multiplicable for usize {
  fn initial_c (a : &Matrix<Self>, b : &Matrix<Self>) -> Matrix<Self> {
    a.iter().map(|_| b[0].iter().map(|_| 0).collect()).collect()
  }
  fn singleton_matrix(a : Self, b : Self, c : Self) -> Self {
    c + a * b
  }
  fn neutral_matrix (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect()
  }
}

impl Sendable for f64 {}
impl Multiplicable for f64 {
  fn initial_c (a : &Matrix<Self>, b : &Matrix<Self>) -> Matrix<Self> {
    a.iter().map(|_| b[0].iter().map(|_| 0.0).collect()).collect()
  }
  fn singleton_matrix(a : Self, b : Self, c : Self) -> Self {
    c + a * b
  }
  fn neutral_matrix (rows : usize, cols : usize) -> Matrix<Self> {
    (0..rows).map(|_| (0..cols).map(|_| 0.0).collect()).collect()
  }
}

impl<T:Sendable> Sendable for Vec<Vec<T>> {}
pub type Matrix<T> = Vec<Vec<T>>;

impl Sendable for Duration {}

impl<X:Sendable> Sendable for Option<X> {}

impl<X:Sendable, Y:Sendable> Sendable for (X,Y) {}
impl<X:Sendable, Y:Sendable, Z:Sendable> Sendable for (X,Y,Z) {}
