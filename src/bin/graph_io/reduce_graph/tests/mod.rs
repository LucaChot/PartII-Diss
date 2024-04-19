use crate::types::chain::Chain;

use super::*;


#[test]
fn test_connected_true(){
  let a = Edge::new(0,1,1.0);
  let b = Edge::new(0,2,1.0);

  let result = Rc::new(a).is_connected(&Rc::new(b));

  assert_eq!(result, Some((true,true)));
}

#[test]
fn test_connected_true2(){
  let a = Edge::new(0,5,1.0);
  let b = Edge::new(5,6,1.0);

  let result = Rc::new(a).is_connected(&Rc::new(b));

  assert_eq!(result, Some((false,true)));
}

#[test]
fn test_connected_false(){
  let a = Edge::new(0,1,1.0);
  let b = Edge::new(2,3,1.0);

  let result = Rc::new(a).is_connected(&Rc::new(b));

  assert_eq!(result, None);
}

//------------------------------------------------------------ 


#[test]
fn test_removal_linear1(){
  let (edges, chains) = remove_val_2_nodes("src/bin/graph_io/reduce_graph/tests/graphs/linear1.txt");

  assert_eq!(*edges[0], Edge::new(0,3,3.0));

  assert_eq!(chains[0].start, 0);
  assert_eq!(chains[0].end, 3);
  assert_eq!(chains[0].dist, 3.0);
  assert_eq!(chains[0].nodes, vec![(0,0.0),(1,1.0),(2,2.0),(3,3.0)]);
}

#[test]
fn test_removal_linear2(){
  let (edges, chains) = remove_val_2_nodes("src/bin/graph_io/reduce_graph/tests/graphs/linear2.txt");

  assert_eq!(*edges[0], Edge::new(0,2,2.0));

  assert_eq!(*edges[1], Edge::new(0,3,1.0));

  assert_eq!(chains[0].start, 0);
  assert_eq!(chains[0].end, 2);
  assert_eq!(chains[0].dist, 2.0);
  assert_eq!(chains[0].nodes, vec![(0,0.0),(1,1.0),(2,2.0)]);
}

#[test]
fn test_removal_linear3(){
  let (edges, chains) = remove_val_2_nodes("src/bin/graph_io/reduce_graph/tests/graphs/linear3.txt");

  assert_eq!(*edges[0], Edge::new(0,2,3.0));

  assert_eq!(chains[0].start, 0);
  assert_eq!(chains[0].end, 2);
  assert_eq!(chains[0].dist, 3.0);
  assert_eq!(chains[0].nodes, vec![(0,0.0),(3,1.0),(1,2.0),(2,3.0)]);
}

#[test]
fn test_removal_cycle(){
  let (edges, chains) = remove_val_2_nodes("src/bin/graph_io/reduce_graph/tests/graphs/cycle.txt");

  assert_eq!(*edges[0], Edge::new(0,2,2.0));
  assert_eq!(*edges[1], Edge::new(0,2,3.0));
  assert_eq!(*edges[2], Edge::new(2,6,1.0));
  assert_eq!(*edges[3], Edge::new(0,5,1.0));

  let mut chain= &chains[0];
  assert_eq!(chain.start, 0);
  assert_eq!(chain.end, 2);
  assert_eq!(chain.dist, 2.0);
  assert_eq!(chain.nodes, vec![(0,0.0),(1,1.0),(2,2.0)]);

  chain = &chains[1];
  assert_eq!(chain.start, 2);
  assert_eq!(chain.end, 0);
  assert_eq!(chain.dist, 3.0);
  assert_eq!(chain.nodes, vec![(2,0.0),(4,1.0),(3,2.0),(0,3.0)]);
}

#[test]
fn test_removal_complex(){
  let (edges, chains) = remove_val_2_nodes("src/bin/graph_io/reduce_graph/tests/graphs/complex.txt");

  dbg!(&chains);

  assert_eq!(*edges[0], Edge::new(0,5,1.0));
  assert_eq!(*edges[1], Edge::new(4,5,1.0));
  assert_eq!(*edges[2], Edge::new(5,6,1.0));
  assert_eq!(*edges[3], Edge::new(1,6,2.0));
  assert_eq!(*edges[4], Edge::new(3,6,1.0));

  let chain= &chains[0];
  assert_eq!(chain.start, 6);
  assert_eq!(chain.end, 1);
  assert_eq!(chain.dist, 2.0);
  assert_eq!(chain.nodes, vec![(6,0.0),(2,1.0),(1,2.0)]);
}

#[test]
fn test_remove_redundancy(){
  let (mut edges, node_num) = edge_file_to_vec("src/bin/graph_io/reduce_graph/tests/graphs/redundant.txt");
  let mut nodes = create_node_vec(&edges, node_num);
  sort_edges_by_nodes(&mut edges);
  dbg!(&edges);
  let deduplicated = updated_reduced_edges(&edges, &mut nodes);
  dbg!(&deduplicated);
  let correct = vec![
    Edge::new(0,1,1.0),
    Edge::new(1,2,1.0),
    Edge::new(1,5,4.0),
    Edge::new(2,3,1.0),
  ];

  for (dedup_edge, correct_edge) in deduplicated.iter().zip(correct){
    assert_eq!(*(*dedup_edge), correct_edge);
  }
}

#[test]
fn test_store_val2() {
  //let (_, chains) = remove_val_2_nodes("src/bin/graph_io/reduce_graph/tests/graphs/cycle.txt");
  //assert_eq!(
  //  store_chains_txt(chains, "src/bin/graph_io/reduce_graph/tests/results/cycle_nodes.txt")
  //  , ());
}
