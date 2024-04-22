use std::fs::File;
use std::io::{self, Write, Read};

use sim::types::{Matrix, Msg};

pub trait Store : Sized {
   fn store(&self,  output_file_path : &str) -> io::Result<()> ;
   fn load(&mut self, input_file_path : &str) -> io::Result<()>;
   fn num_nodes(&self) -> usize;
}


impl Store for Matrix<Msg> {
  fn store(&self,  output_file_path : &str) -> io::Result<()> {
    let mut file = File::create(output_file_path)?;
    let serialized = bincode::serialize(self).unwrap();
    file.write_all(&serialized)?;
    Ok(())
  }

  fn load(&mut self, input_file_path : &str) -> io::Result<()> {
    let mut file = File::open(input_file_path)?;
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    match bincode::deserialize(&buffer) {
      Ok(data) => {
        *self = data;
        Ok(())
      }
      Err(err) => {
        let err_msg = format!("Failed to deserialize data: {}", err);
        Err(io::Error::new(io::ErrorKind::InvalidData, err_msg))
      }
    }
  }

  fn num_nodes(&self) -> usize {
      self.len()
  }
}

#[cfg(test)]
mod tests;
  
