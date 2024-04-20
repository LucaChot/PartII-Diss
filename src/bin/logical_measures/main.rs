use clap_derive::ValueEnum;
use sim::matmul::comm_method::{Hash, FoxOtto, Cannon};
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
    
    /// File to write json to
    #[arg(short, long)]
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
}

impl CliComm {
  fn display(&self) -> &str {
    match self {
      Self::Hash => "Hash",
      Self::FoxOtto => "FoxOtto",
      Self::Cannon => "Cannon",
    }
  }
}
  

static mut ITERATIONS : usize = 20;

fn main() -> std::io::Result<()> {
  let cli = Cli::parse();

  unsafe {
    ITERATIONS = cli.iter;
  }

  let group = match cli.command {
    Command::Matrix { start, end, step, proc} => {
      let matrix_sizes = (start..=end).step_by(step);
      match cli.comm {
        None => against_matrices_all(proc, matrix_sizes),
        Some(comm) => {
          let mut g = Group::new(format!("{} vs Matrix size", comm.display()));
          match comm {
            CliComm::Hash => {
              g.data.push(against_matrices::<Hash>(proc, matrix_sizes));
              g
            },
            CliComm::FoxOtto => {
              g.data.push(against_matrices::<FoxOtto>(proc, matrix_sizes));
              g
            },
            CliComm::Cannon => {
              g.data.push(against_matrices::<Cannon>(proc, matrix_sizes));
              g
            }
          }
        }
      }
    },
    Command::Processor { start, end, step, matrix} => {
      let proc_sizes = (start..=end).step_by(step);
      match cli.comm {
        None => against_processor_all(proc_sizes, matrix),
        Some(comm) => {
          let mut g = Group::new(format!("{} vs Processor size", comm.display()));
          match comm {
            CliComm::Hash => {
              g.data.push(against_processor::<Hash>(proc_sizes, matrix));
              g
            },
            CliComm::FoxOtto => {
              g.data.push(against_processor::<FoxOtto>(proc_sizes, matrix));
              g
            },
            CliComm::Cannon => {
              g.data.push(against_processor::<Cannon>(proc_sizes, matrix));
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
  let mut file = File::create(cli.output)?;
  file.write_all(json_data.as_bytes())?;

  println!("Data has been written to data.json");
  Ok(())
}
