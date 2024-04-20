use std::fmt;
use std::io::{self, Write, BufRead};
use std::error::Error;
use std::fs::File;
use std::num::{ParseFloatError, ParseIntError};
use std::rc::Rc;
use std::cell::RefCell;

use sim::matmul::Multiplicable;
use sim::types::{Matrix, Msg};

use crate::adj_matrix::Store;

#[derive(Debug)]
pub struct Edge {
    pub node_a : usize,
    pub node_b : usize,
    pub distance : f64,
    pub visited: RefCell<bool>,
    pub order: RefCell<usize>,
    pub from_chain : Option<usize>
}

impl Edge {
  pub fn new(node_a : usize, node_b : usize, distance : f64) -> Self {
      Edge { node_a,
        node_b,
        distance,
        visited : RefCell::new(false),
        order :  RefCell::new(0),
        from_chain : None
      }
  }

  pub fn from_chain(node_a : usize, node_b : usize, distance : f64, chain : usize) -> Self {
      Edge { node_a,
        node_b,
        distance,
        visited : RefCell::new(false),
        order :  RefCell::new(0),
        from_chain : Some(chain),
      }
  }

  pub fn is_connected(&self, b : &Edge) -> Option<(bool,bool)>{ 
    match self.node_a {
      n if n == b.node_a => Some((true, true)),
      n if n == b.node_b => Some((true, false)),
      _ => match self.node_b {
        n if n == b.node_a => Some((false, true)),
        n if n == b.node_b => Some((false, false)),
        _ => None,
      },
    }
  }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.node_a == other.node_a 
          && self.node_b == other.node_b 
          && self.distance == other.distance
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      match (self.node_a.cmp(&other.node_a),self.node_b.cmp(&other.node_b)) {
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => 
          Some(self.distance.total_cmp(&other.distance)),
        _ => None
      }
    }
}

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

impl Store for Vec<Rc<Edge>> {
  fn store(&self,  output_file_path : &str) -> io::Result<()> {
    let formatted_strings : Vec<String> = self.iter().enumerate()
      .map(|(id, edge)| format!("{} {} {} {}", id, edge.node_a, edge.node_b, edge.distance))
      .collect();

    let mut file = File::create(output_file_path)?;
    for line in formatted_strings{
      writeln!(file, "{}", line)?;
    }
    Ok(())
  }

  fn load(&mut self, input_file_path : &str) -> io::Result<()> {
    let input_file = File::open(input_file_path)?;
    let input_reader = io::BufReader::new(input_file);
    // Iterate over the lines in the file
    for line in input_reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
              match parse_string(&content) {
                Ok((start_node, end_node, distance)) =>  {
                  self.push(Rc::new(Edge::new(start_node, end_node, distance)))
                }
                ,
                Err(err) => eprintln!("Error reading line: {}", err),
              }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
    Ok(())
  }

  fn num_nodes(&self) -> usize {
    let num = self.iter()
      .map(|edge| if edge.node_a > edge.node_b { edge.node_a } else { edge.node_b })
      .max();
    match num {
      None => 0,
      Some(num_nodes) => num_nodes + 1
    }
  }
}

pub trait ToAdj {
  fn into_adj(&self) -> Matrix<Msg>;
}

impl ToAdj for Vec<Rc<Edge>> {
  fn into_adj(&self) -> Matrix<Msg> {
    let num_nodes = self.num_nodes();
    let mut adj = Msg::neutral_matrix(num_nodes, num_nodes);

    for edge in self.iter() {
      adj[edge.node_a][edge.node_b] = Msg::new(edge.distance, edge.node_a);
      adj[edge.node_b][edge.node_a] = Msg::new(edge.distance, edge.node_b);
    }
    adj  
  }
}

#[cfg(test)]
mod tests;
