use clap_derive::ValueEnum;
use sim::{matmul::comm_method::{Hash, FoxOtto, Cannon, PipeFoxOtto}, processor::TaurusNetworkBuilder};
use std::fs::File;
use std::io::prelude::*;

mod bench;
mod commands;
use bench::Group;
use commands::{against_matrices, against_processor, against_matrices_all, against_processor_all};

use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Benchmark to run
    #[command(subcommand)]
    command: Command,

    /// Specify a communication method
    #[arg(value_enum)]
    comm: Option<CliComm>,

    /// Latency of core in ns
    #[arg(short, long, default_value_t = 100)]
    latency : usize,

    /// Bandwidth of core in B/ns
    #[arg(short, long, default_value_t = 100000000)]
    bandwidth : usize,

    /// Startup of broadcast
    #[arg(short, long, default_value_t = 1)]
    startup : usize,
    
    /// File to write json to
    #[arg(short, long, default_value_t = String::from("data.json"))]
    output: String,

    /// Number of iterations per run
    #[arg(short, long, default_value_t = 20)]
    iter: usize,

}

#[derive(Subcommand)]
pub enum Command {
    /// Iterate over matrix sizes
    Matrix {
      /// Starting matrix size
      #[arg(long)]
      start: usize,
      /// Limit matrix size
      #[arg(long)]
      end: usize,
      /// Size of increments
      #[arg(long)]
      step: usize,
      /// Number of cores
      #[arg(long)]
      proc: usize,
    },
    /// Iterate over processor sizes
    Processor {
      /// Starting processor size
      #[arg(long)]
      start: usize,
      /// Limit processor size
      #[arg(long)]
      end: usize,
      /// Size of increments
      #[arg(long)]
      step: usize,
      /// Size of matrices
      #[arg(long)]
      matrix: usize,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliComm {
  /// Simple Broadcast
  Hash,
  /// FoxOtto
  FoxOtto,
  /// Cannon
  Cannon,
  /// Pipelined FoxOtto
  PipeFoxOtto
}

impl CliComm {
  fn display(&self) -> &str {
    match self {
      Self::Hash => "Hash",
      Self::FoxOtto => "FoxOtto",
      Self::Cannon => "Cannon",
      Self::PipeFoxOtto => "Pipeline FoxOtto",
    }
  }
}
  

static mut ITERATIONS : usize = 20;

fn main() -> std::io::Result<()> {
  let cli = Cli::parse();

  unsafe {
    ITERATIONS = cli.iter;
  }

  let network_builder = TaurusNetworkBuilder::new(cli.latency, cli.bandwidth, cli.startup);
  let group = match cli.command {
    Command::Matrix { start, end, step, proc} => {
      let matrix_sizes = (start..=end).step_by(step);
      match cli.comm {
        None => against_matrices_all(proc, matrix_sizes, network_builder),
        Some(comm) => {
          let mut g = Group::new(format!("{} vs Matrix size", comm.display()));
          match comm {
            CliComm::Hash => {
              g.data.push(against_matrices::<Hash>(proc, matrix_sizes,network_builder));
              g
            },
            CliComm::FoxOtto => {
              g.data.push(against_matrices::<FoxOtto>(proc, matrix_sizes,network_builder));
              g
            },
            CliComm::Cannon => {
              g.data.push(against_matrices::<Cannon>(proc, matrix_sizes,network_builder));
              g
            },
            CliComm::PipeFoxOtto => {
              g.data.push(against_matrices::<PipeFoxOtto>(proc, matrix_sizes,network_builder));
              g
            }
          }
        }
      }
    },
    Command::Processor { start, end, step, matrix} => {
      let proc_sizes = (start..=end).step_by(step).map(|x| 2_i32.pow(x as u32) as usize);
      match cli.comm {
        None => against_processor_all(proc_sizes, matrix, network_builder),
        Some(comm) => {
          let mut g = Group::new(format!("{} vs Processor size", comm.display()));
          match comm {
            CliComm::Hash => {
              g.data.push(against_processor::<Hash>(proc_sizes, matrix, network_builder));
              g
            },
            CliComm::FoxOtto => {
              g.data.push(against_processor::<FoxOtto>(proc_sizes, matrix, network_builder));
              g
            },
            CliComm::Cannon => {
              g.data.push(against_processor::<Cannon>(proc_sizes, matrix, network_builder));
              g
            },
            CliComm::PipeFoxOtto => {
              g.data.push(against_processor::<PipeFoxOtto>(proc_sizes, matrix, network_builder));
              g
            }
          }
        }
      }
    }
  };

  // Convert the data to JSON format
  let json_data = serde_json::to_string(&group)?;

  // Write the JSON data to a file
  let mut file = File::create(&cli.output)?;
  file.write_all(json_data.as_bytes())?;

  println!("Data has been written to {}", &cli.output);
  Ok(())
}
