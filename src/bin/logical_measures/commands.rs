use std::any::type_name;

use sim::{matmul::{MatMul, comm_method::{Hash, CommMethod, FoxOtto, Cannon, PipeFoxOtto}}, types::Matrix};
use sim::processor::{Processor, TaurusNetworkBuilder, TaurusCoreInfo};
use crate::bench::{Run, Bench, Group};
use crate::ITERATIONS;


pub fn against_processor<T>(proc_sizes : impl Iterator<Item = usize>,
                            matrix_size : usize,
                            network_builder : TaurusNetworkBuilder) -> Bench
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
      let mut processor = Processor::new(processor_size,processor_size, Box::new(network_builder));
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

pub fn against_processor_all(proc_sizes : impl Iterator<Item = usize> + Clone
                             , matrix_size : usize,
                             network_builder : TaurusNetworkBuilder) -> Group {
  let mut group = Group::new(format!("All vs Processor"));
  println!("Running {group}");
  group.data.push(against_processor::<Hash>(proc_sizes.clone(), matrix_size, network_builder));
  group.data.push(against_processor::<FoxOtto>(proc_sizes.clone(), matrix_size, network_builder));
  group.data.push(against_processor::<Cannon>(proc_sizes.clone(), matrix_size, network_builder));
  group.data.push(against_processor::<PipeFoxOtto>(proc_sizes.clone(), matrix_size, network_builder));
  group
}

pub fn against_matrices<T>(proc_size : usize,
                           matrix_sizes : impl Iterator<Item = usize>
                           , network_builder : TaurusNetworkBuilder) -> Bench
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
      let mut processor = Processor::new(proc_size,proc_size, Box::new(network_builder));
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

pub fn against_matrices_all(proc_size : usize, 
                            matrix_sizes : impl Iterator<Item=usize> + Clone,
                            network_builder : TaurusNetworkBuilder) -> Group {
  let mut group = Group::new(format!("All vs Matrices"));
  println!("Running {group}");
  group.data.push(against_matrices::<Hash>(proc_size, matrix_sizes.clone(), network_builder));
  group.data.push(against_matrices::<FoxOtto>(proc_size, matrix_sizes.clone(),network_builder));
  group.data.push(against_matrices::<Cannon>(proc_size, matrix_sizes.clone(),network_builder));
  group.data.push(against_matrices::<PipeFoxOtto>(proc_size, matrix_sizes.clone(),network_builder));
  group
}
