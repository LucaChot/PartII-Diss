use Simulator::MatMul;
use Simulator::matrix_multiplication::Cannon;
use Simulator::processor::{Processor, TaurusNetworkBuilder};

fn main() {
  const MATRIX_WIDTH : usize = 125;
  const MATRIX_HEIGHT : usize = 125;
  const PROCESSOR_WIDTH : usize = 10;
  const PROCESSOR_HEIGHT : usize = 10;
  let a = vec![vec![0; MATRIX_WIDTH]; MATRIX_HEIGHT];
  //let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
  let mut processor = Processor::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH, Box::new(TaurusNetworkBuilder::new()));
  let mut matmul : MatMul<isize> = MatMul::new(&mut processor);
  let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
  matmul.parallel_square::<Cannon>(a, iterations);
  processor.display_processor_time();
}
