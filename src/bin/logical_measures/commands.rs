use std::{any::type_name, time::Duration};

use sim::{matmul::{MatMul, comm_method::{Hash, CommMethod, FoxOtto, Cannon}}, types::Matrix};
use sim::processor::{Processor, TaurusNetworkBuilder, TaurusCoreInfo};
use crate::bench::{Run, Bench, Group};
use crate::ITERATIONS;


pub fn against_processor<T>(proc_sizes : impl Iterator<Item = usize>, matrix_size : usize) -> Bench
where T : CommMethod<isize, TaurusCoreInfo<Matrix<isize>>> {
  let mut bench = Bench::new(format!("{} vs Processor", type_name::<T>()));
  println!("Running {bench}");
  for processor_size in proc_sizes {
    let mut run = Run::new(matrix_size, processor_size);
    let iter : usize;
    unsafe {
      iter = ITERATIONS;
    }
    for _ in 0..iter {
      let a = vec![vec![0; matrix_size]; matrix_size];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let network_builder = TaurusNetworkBuilder::new(Duration::ZERO, 1000000000, Duration::ZERO);
      let mut processor = Processor::new(2,2, Box::new(network_builder));
      let mut matmul : MatMul<isize> = MatMul::new(&mut processor);
      matmul.parallel_square::<T>(a,iterations);
      match processor.max_debug_time() {
        Some(time) => run.data.push(time),
        _ => ()
      };
    }
    bench.data.push(run);
  }
  bench
}

pub fn against_processor_all(proc_sizes : impl Iterator<Item = usize> + Clone, matrix_size : usize) -> Group {
  let mut group = Group::new(format!("All vs Processor"));
  println!("Running {group}");
  group.data.push(against_processor::<Hash>(proc_sizes.clone(), matrix_size));
  group.data.push(against_processor::<FoxOtto>(proc_sizes.clone(), matrix_size));
  group.data.push(against_processor::<Cannon>(proc_sizes.clone(), matrix_size));
  group
}

pub fn against_matrices<T>(proc_size : usize, matrix_sizes : impl Iterator<Item = usize>) -> Bench
where T : CommMethod<isize, TaurusCoreInfo<Matrix<isize>>> {
  let mut bench = Bench::new(format!("{} vs Matrices", type_name::<T>()));
  println!("Running {bench}");
  for matrix_size in matrix_sizes {
    let mut run = Run::new(matrix_size, proc_size);
    let iter : usize;
    unsafe {
      iter = ITERATIONS;
    }
    for _ in 0..iter {
      let a = vec![vec![0; matrix_size]; matrix_size];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let network_builder = TaurusNetworkBuilder::new(Duration::ZERO, 1000000000, Duration::ZERO);
      let mut processor = Processor::new(2,2, Box::new(network_builder));
      let mut matmul : MatMul<isize> = MatMul::new(&mut processor);
      matmul.parallel_square::<T>(a,iterations);
      match processor.max_debug_time() {
        Some(time) => run.data.push(time),
        _ => ()
      };
    }
    bench.data.push(run);
  }
  bench
}

pub fn against_matrices_all(proc_size : usize, matrix_sizes : impl Iterator<Item=usize> + Clone) -> Group {
  let mut group = Group::new(format!("All vs Matrices"));
  println!("Running {group}");
  group.data.push(against_matrices::<Hash>(proc_size, matrix_sizes.clone()));
  group.data.push(against_matrices::<FoxOtto>(proc_size, matrix_sizes.clone()));
  group.data.push(against_matrices::<Cannon>(proc_size, matrix_sizes.clone()));
  group
}
