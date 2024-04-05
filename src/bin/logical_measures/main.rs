use Simulator::{Comm, Algorithm};

fn main() {
  const MATRIX_WIDTH : usize = 16;
  const MATRIX_HEIGHT : usize = 16;
  const PROCESSOR_WIDTH : usize = 3;
  const PROCESSOR_HEIGHT : usize = 3;
  let a = vec![vec![1; MATRIX_WIDTH]; MATRIX_HEIGHT];
  //let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
  let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
  let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
  p.parallel_square(a, iterations, Comm::CANNON);
}
