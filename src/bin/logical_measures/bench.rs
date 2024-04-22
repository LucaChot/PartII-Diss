use serde::{Serialize, Deserialize};
use std::fmt;

// Define a struct representing your data
#[derive(Serialize, Deserialize)]
pub struct Run {
    pub matrix_size : usize,
    pub processor_size : usize,
    pub data: Vec<u128>,
}

impl Run {
  pub fn new(matrix_size : usize, processor_size : usize) -> Self {
    Run { matrix_size, processor_size, data : Vec::new() }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Bench {
    pub name: String,
    pub data: Vec<Run>,
}

impl Bench {
  pub fn new(name : String) -> Self {
    Bench { name, data : Vec::new() }
  }
}

impl fmt::Display for Bench {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Group {
  pub name : String,
  pub data : Vec<Bench>
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

#[derive(Serialize, Deserialize)]
pub struct Groups {
  pub data : Vec<Group>
}

impl Groups {
  pub fn new() -> Self {
    Groups { data: Vec::new() }
  }
}
