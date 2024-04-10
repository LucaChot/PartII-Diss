use Simulator::{Comm, Algorithm};

fn main() {
  const MATRIX_WIDTH : usize = 125;
  const MATRIX_HEIGHT : usize = 125;
  const PROCESSOR_WIDTH : usize = 10;
  const PROCESSOR_HEIGHT : usize = 10;
  let a = vec![vec![0; MATRIX_WIDTH]; MATRIX_HEIGHT];
  //let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
  let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
  let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
  p.parallel_square(a, iterations, Comm::CANNON);
}
