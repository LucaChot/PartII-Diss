use std::rc::Rc;
use sim::types::Msg;
use crate::{types::edge::Edge, adj_matrix::Store};

use super::ToAdj;


#[test]
fn load_singleton_ingraph(){
  let state_file = "src/bin/graph_io/types/edge/tests/simple.txt";
  let mut edge : Vec<Rc<Edge>> = Vec::new();

  let _ = edge.load(state_file);
  let adj = edge.into_adj();

  let (matrix_w, matrix_p) = Msg::unzip(&adj);

  assert_eq!(matrix_w, 
             vec![[0.0,1.0,2.0],
                  [1.0,0.0,3.0],
                  [2.0,3.0,0.0]]);
  assert_eq!(matrix_p, 
             vec![[0,0,0],
                  [1,1,1],
                  [2,2,2]]);
}
