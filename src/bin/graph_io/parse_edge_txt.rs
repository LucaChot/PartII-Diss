use std::fmt;
use std::io::{self, Write, BufRead};
use std::error::Error;
use std::fs::File;
use std::num::{ParseFloatError, ParseIntError};
use std::rc::Rc;

use serde::Serialize;

use crate::types::{Edge, Chain};

// Define a custom error enum that wraps ParseFloatError and ParseIntError
#[derive(Debug)]
pub enum ParseError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
    Empty
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::IntError(err)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> Self {
        ParseError::FloatError(err)
    }
}

// Implement the Error trait for ParseError
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FloatError(err) => write!(f, "Float parsing error: {}", err),
            ParseError::IntError(err) => write!(f, "Int parsing error: {}", err),
            ParseError::Empty => write!(f,"Missing items"),
        }
    }
}

impl Error for ParseError {}

fn parse_string(values : &str) -> Result<(usize, usize, f64), ParseError> {
    let mut parts = values.split_whitespace();
    parts.next();
    let first_usize: usize = parts.next()
      .ok_or(ParseError::Empty)?
      .parse()?;
    let second_usize: usize = parts.next()
      .ok_or(ParseError::Empty)?
      .parse()?;
    let float_value: f64 = parts.next()
      .ok_or(ParseError::Empty)?
      .parse()?;

    Ok((first_usize, second_usize, float_value))
}

pub fn edge_file_to_vec(input_file_name : &str) -> (Vec<Rc<Edge>>, usize) {
  let input_file = File::open(input_file_name).unwrap();
  let input_reader = io::BufReader::new(input_file);
  let mut edges : Vec<Rc<Edge>> = Vec::new();
  let mut num_nodes = 0;
  // Iterate over the lines in the file
  for line in input_reader.lines() {
      // Handle each line
      match line {
          Ok(content) => {
            match parse_string(&content) {
              Ok((start_node, end_node, distance)) =>  {
                if end_node >= num_nodes {
                  num_nodes = end_node + 1;
                }
                edges.push(Rc::new(Edge::new(start_node, end_node, distance)))
              }
              ,
              Err(err) => eprintln!("Error reading line: {}", err),
            }
          }
          Err(err) => eprintln!("Error reading line: {}", err),
      }
  }
  (edges, num_nodes)
}

pub fn store_edges(edges : Vec<Rc<Edge>>, output_file_path : &str)  {
  let formatted_strings : Vec<String> = edges.into_iter().enumerate()
    .map(|(id, edge)| format!("{} {} {} {}", id, edge.node_a, edge.node_b, edge.distance))
    .collect();

  let mut file = File::create(output_file_path).unwrap();
  for line in formatted_strings{
    writeln!(file, "{}", line).unwrap();
  }
}

pub fn store_vec<T : Serialize> (items : &Vec<T>, output_file_path : &str) {
  let mut file = File::create(output_file_path).unwrap();
  for item in items {
    let serialized = bincode::serialize(&item).unwrap();
    file.write_all(&serialized).unwrap();
  }
}

pub fn store_chains_txt(chains : Vec<Chain>, output_file_path : &str) {
  let formatted_strings : Vec<String> = chains.into_iter().enumerate()
    .flat_map(|(id, chain)| chain.chain_to_io(id))
    .collect();

  let mut file = File::create(output_file_path).unwrap();
  for line in formatted_strings{
    writeln!(file, "{}", line).unwrap();
  }
}
