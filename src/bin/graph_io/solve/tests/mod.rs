use std::rc::Rc;
use sim::{types::Msg, processor::{TaurusNetworkBuilder, Processor}, matmul::{MatMul, comm_method::Hash}};
use crate::{solve::Path, types::edge::{Edge, ToAdj}, adj_matrix::Store};

use super::Solver;


#[test]
fn test_solve_linear1(){
  let input_file_path = "src/bin/graph_io/test_graphs/linear1.txt";
  //let edge_file_path = "src/bin/graph_io/solve/tests/results/linear1_edge.txt";
  let node_file_path = "src/bin/graph_io/solve/tests/results/linear1_node";
  let chain_file_path = "src/bin/graph_io/solve/tests/results/linear1_chain";

  /*
  match complete_reduction(input_file_path, edge_file_path, node_file_path, chain_file_path) {
    Ok(()) => (),
    Err(e) => {
      dbg!(e);
      panic!();
    }
  }
  */

  let mut edge_old : Vec<Rc<Edge>> = Vec::new();
  edge_old.load(input_file_path).unwrap();
  let num_nodes = edge_old.num_nodes();

  let w_matrix = vec![ vec![0.0,3.0],
                       vec![3.0,0.0]];

  let p_matrix = vec![ vec![0,0],
                       vec![1,1]];

  let adj = Msg::zip(&w_matrix, &p_matrix);
  let mut s = Solver::new(adj, node_file_path, chain_file_path, num_nodes).unwrap();

  assert_eq!(s.find_path(0,3), Path::new(3.0, vec![0,1,2,3]));
  assert_eq!(s.find_path(3,0), Path::new(3.0, vec![3,2,1,0]));
  assert_eq!(s.find_path(1,2), Path::new(1.0, vec![1,2]));
  assert_eq!(s.find_path(1,3), Path::new(2.0, vec![1,2,3]));
  assert_eq!(s.find_path(2,0), Path::new(2.0, vec![2,1,0]));
  
}

#[test]
fn test_solve_linear2(){
  let input_file_path = "src/bin/graph_io/test_graphs/linear2.txt";
  //let edge_file_path = "src/bin/graph_io/solve/tests/results/linear2_edge.txt";
  let node_file_path = "src/bin/graph_io/solve/tests/results/linear2_node";
  let chain_file_path = "src/bin/graph_io/solve/tests/results/linear2_chain";

  /*
  match complete_reduction(input_file_path, edge_file_path, node_file_path, chain_file_path) {
    Ok(()) => (),
    Err(e) => {
      dbg!(e);
      panic!();
    }
  }
  */
  let mut edge_old : Vec<Rc<Edge>> = Vec::new();
  edge_old.load(input_file_path).unwrap();
  let num_nodes = edge_old.num_nodes();

  let w_matrix = vec![ vec![0.0,2.0,1.0],
                       vec![2.0,0.0, 3.0],
                       vec![1.0,3.0, 0.0],
  ];

  let p_matrix = vec![ vec![0,0,0],
                       vec![1,1,0],
                       vec![2,0,2]
  ];

  let adj = Msg::zip(&w_matrix, &p_matrix);
  let mut s = Solver::new(adj, node_file_path, chain_file_path, num_nodes).unwrap();

  assert_eq!(s.find_path(3,2), Path::new(3.0, vec![3,0,1,2]));
  assert_eq!(s.find_path(2,3), Path::new(3.0, vec![2,1,0,3]));
  assert_eq!(s.find_path(1,0), Path::new(1.0, vec![1,0]));
  assert_eq!(s.find_path(1,3), Path::new(2.0, vec![1,0,3]));
  assert_eq!(s.find_path(2,0), Path::new(2.0, vec![2,1,0]));
}

#[test]
fn test_solve_linear3(){
  let input_file_path = "src/bin/graph_io/test_graphs/linear3.txt";
  //let edge_file_path = "src/bin/graph_io/solve/tests/results/linear3_edge.txt";
  let node_file_path = "src/bin/graph_io/solve/tests/results/linear3_node";
  let chain_file_path = "src/bin/graph_io/solve/tests/results/linear3_chain";

  /*
  match complete_reduction(input_file_path, edge_file_path, node_file_path, chain_file_path) {
    Ok(()) => (),
    Err(e) => {
      dbg!(e);
      panic!();
    }
  }
  */
  let mut edge_old : Vec<Rc<Edge>> = Vec::new();
  edge_old.load(input_file_path).unwrap();
  let num_nodes = edge_old.num_nodes();

  let w_matrix = vec![ vec![0.0,3.0],
                       vec![3.0,0.0]];

  let p_matrix = vec![ vec![0,0],
                       vec![1,1]];

  let adj = Msg::zip(&w_matrix, &p_matrix);
  let mut s = Solver::new(adj, node_file_path, chain_file_path, num_nodes).unwrap();

  assert_eq!(s.find_path(0,2), Path::new(3.0, vec![0,3,1,2]));
  assert_eq!(s.find_path(2,0), Path::new(3.0, vec![2,1,3,0]));
  assert_eq!(s.find_path(1,3), Path::new(1.0, vec![1,3]));
  assert_eq!(s.find_path(1,2), Path::new(1.0, vec![1,2]));
  assert_eq!(s.find_path(0,1), Path::new(2.0, vec![0,3,1]));
}

#[test]
fn test_solve_cycle(){
  let input_file_path = "src/bin/graph_io/test_graphs/cycle.txt";
  //let edge_file_path = "src/bin/graph_io/solve/tests/results/cycle_edge.txt";
  let node_file_path = "src/bin/graph_io/solve/tests/results/cycle_node";
  let chain_file_path = "src/bin/graph_io/solve/tests/results/cycle_chain";

  /*
  match complete_reduction(input_file_path, edge_file_path, node_file_path, chain_file_path) { 
    Ok(()) => (),
    Err(e) => {
      dbg!(e);
      panic!();
    }
  }
  */
  let mut edge_old : Vec<Rc<Edge>> = Vec::new();
  edge_old.load(input_file_path).unwrap();
  let num_nodes = edge_old.num_nodes();

  let w_matrix = vec![vec![0.0,2.0,1.0,3.0],
                      vec![2.0,0.0,3.0,1.0],
                      vec![1.0,3.0,0.0,4.0],
                      vec![3.0,1.0,4.0,0.0],
  ];

  let p_matrix = vec![vec![0,0,0,1],
                      vec![1,1,0,1],
                      vec![2,0,2,1],
                      vec![1,3,0,3],
  ];

  let adj = Msg::zip(&w_matrix, &p_matrix);
  let mut s = Solver::new(adj, node_file_path, chain_file_path, num_nodes).unwrap();

  assert_eq!(s.find_path(5,6), Path::new(4.0, vec![5,0,1,2,6]));
  assert_eq!(s.find_path(1,3), Path::new(2.0, vec![1,0,3]));
  assert_eq!(s.find_path(3,6), Path::new(3.0, vec![3,4,2,6]));
  assert_eq!(s.find_path(3,4), Path::new(1.0, vec![3,4]));
  assert_eq!(s.find_path(4,5), Path::new(3.0, vec![4,3,0,5]));
}

#[test]
fn test_solve_complex(){
  let input_file_path = "src/bin/graph_io/test_graphs/complex.txt";
  let edge_file_path = "src/bin/graph_io/solve/tests/results/complex_edge.txt";
  let node_file_path = "src/bin/graph_io/solve/tests/results/complex_node";
  let chain_file_path = "src/bin/graph_io/solve/tests/results/complex_chain";

  /*
  match complete_reduction(input_file_path, edge_file_path, node_file_path, chain_file_path) { 
    Ok(()) => (),
    Err(e) => {
      dbg!(e);
      panic!();
    }
  }
  */
  let mut edge_old : Vec<Rc<Edge>> = Vec::new();
  edge_old.load(input_file_path).unwrap();
  let num_nodes = edge_old.num_nodes();

  let mut edge : Vec<Rc<Edge>> = Vec::new();
  edge.load(edge_file_path).unwrap();

  let adj = edge.into_adj();
  dbg!(&adj);

  let iterations = f64::ceil(f64::log2(adj.len() as f64)) as usize;
  dbg!(&iterations);
  let mut processor = Processor::new(2, 2, Box::new(TaurusNetworkBuilder::new()));
  let mut matmul : MatMul<Msg> = MatMul::new(&mut processor);
  let c = matmul.parallel_square::<Hash>(adj,iterations);
  dbg!(&c);
  
  let mut s = Solver::new(c, node_file_path, chain_file_path, num_nodes).unwrap();

  assert_eq!(s.find_path(0,2), Path::new(3.0, vec![0,5,6,2]));
  assert_eq!(s.find_path(1,3), Path::new(3.0, vec![1,2,6,3]));
  assert_eq!(s.find_path(1,0), Path::new(4.0, vec![1,2,6,5,0]));
  assert_eq!(s.find_path(3,4), Path::new(3.0, vec![3,6,5,4]));
  assert_eq!(s.find_path(2,2), Path::new(0.0, vec![2]));
  assert_eq!(s.find_path(0,0), Path::new(0.0, vec![0]));
}
