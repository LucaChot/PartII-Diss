use sim::matmul::{MatMul, comm_method::{Hash, CommMethod, FoxOtto, Cannon}};
use sim::processor::{Processor, TaurusNetworkBuilder, TaurusCoreInfo};
use sim::types::Matrix;

use serde::{Serialize, Deserialize};
use std::any::type_name;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

const ITERATIONS : usize = 20;

// Define a struct representing your data
#[derive(Serialize, Deserialize)]
struct Run {
    matrix_size : usize,
    processor_size : usize,
    data: Vec<u128>,
}

impl Run {
  pub fn new(matrix_size : usize, processor_size : usize) -> Self {
    Run { matrix_size, processor_size, data : Vec::new() }
  }
}

#[derive(Serialize, Deserialize)]
struct Bench {
    name: String,
    data: Vec<Run>,
}

impl Bench {
  pub fn new(name : String) -> Self {
    Bench { name, data : Vec::new() }
  }
}

#[derive(Serialize, Deserialize)]
struct Group {
  name : String,
  data : Vec<Bench>
}

impl fmt::Display for Bench {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Group {
  pub fn new(name: String) -> Self {
    Group { name, data: Vec::new() }
  }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn against_processor<T>(proc_sizes : Range<usize>, matrix_size : usize) -> Bench
where T : CommMethod<isize, TaurusCoreInfo<Matrix<isize>>> {
  let mut bench = Bench::new(format!("{} Processor", type_name::<T>()));
  println!("Running {bench}");
  for processor_size in proc_sizes {
    let mut run = Run::new(matrix_size, processor_size);
    for _ in 0..ITERATIONS {
      let a = vec![vec![0; matrix_size]; matrix_size];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut processor = Processor::new(processor_size, processor_size, Box::new(TaurusNetworkBuilder::new()));
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
fn against_processor_all(proc_sizes : Range<usize>, matrix_size : usize) -> Group {
  let mut group = Group::new(format!("All vs Processor"));
  println!("Running {group}");
  group.data.push(against_processor::<Hash>(proc_sizes.clone(), matrix_size));
  group.data.push(against_processor::<FoxOtto>(proc_sizes.clone(), matrix_size));
  group.data.push(against_processor::<Cannon>(proc_sizes.clone(), matrix_size));
  group
}

fn against_matrices<T>(proc_size : usize, matrix_sizes : Range<usize>) -> Bench
where T : CommMethod<isize, TaurusCoreInfo<Matrix<isize>>> {
  let mut bench = Bench::new(format!("{} Processor", type_name::<T>()));
  for matrix_size in matrix_sizes {
    let mut run = Run::new(matrix_size, proc_size);
    for _ in 0..ITERATIONS {
      let a = vec![vec![0; matrix_size]; matrix_size];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut processor = Processor::new(proc_size, proc_size, Box::new(TaurusNetworkBuilder::new()));
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

fn main() -> std::io::Result<()> {
  let sizes = 2..10;
  
  let group = against_processor_all(sizes, 100);
  // Convert the data to JSON format
  let json_data = serde_json::to_string(&group)?;

  // Write the JSON data to a file
  let mut file = File::create("data.json")?;
  file.write_all(json_data.as_bytes())?;

  println!("Data has been written to data.json");
  Ok(())
}
