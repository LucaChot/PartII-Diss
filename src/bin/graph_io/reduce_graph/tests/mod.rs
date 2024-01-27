use super::*;

#[test]
fn test_order_linear1(){
    let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/linear1.txt").unwrap();

    let edges = sort_val_2(input_file);
    let edge_order = edges.iter()
      .map(|edge| (edge.node_a, edge.node_b))
      .collect::<Vec<(usize, usize)>>();

  assert_eq!(edge_order, vec![(0,1),(1,2),(2,3)]);
}

#[test]
fn test_order_linear2(){
    let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/linear2.txt").unwrap();

    let edges = sort_val_2(input_file);
    let edge_order = edges.iter()
      .map(|edge| (edge.node_a, edge.node_b))
      .collect::<Vec<(usize, usize)>>();

  assert_eq!(edge_order, vec![(0,1),(1,2),(0,3)]);
}

#[test]
fn test_order_linear3(){
    let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/linear3.txt").unwrap();

    let edges = sort_val_2(input_file);
    let edge_order = edges.iter()
      .map(|edge| (edge.node_a, edge.node_b))
      .collect::<Vec<(usize, usize)>>();

  assert_eq!(edge_order, vec![(0,3),(1,3),(1,2)]);
}

#[test]
fn test_order_cycle(){
    let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/cycle.txt").unwrap();

    let edges = sort_val_2(input_file);
    let edge_order = edges.iter()
      .map(|edge| (edge.node_a, edge.node_b))
      .collect::<Vec<(usize, usize)>>();

  assert_eq!(edge_order, vec![(0,1),(1,2),(2,4),(3,4),(0,3),(2,6),(0,5)]);
}

#[test]
fn test_order_complex(){
    let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/complex.txt").unwrap();

    let edges = sort_val_2(input_file);
    let edge_order = edges.iter()
      .map(|edge| (edge.node_a, edge.node_b))
      .collect::<Vec<(usize, usize)>>();

  assert_eq!(edge_order, vec![(0,5),(4,5),(5,6),(2,6),(1,2),(3,6)]);
}

//------------------------------------------------------------ 

#[test]
fn test_connected_true(){
  let a = Edge::new(0,1,1.0);
  let b = Edge::new(0,2,1.0);

  let result = check_connected(&Rc::new(a), &Rc::new(b));

  assert_eq!(result, Some((true,true)));
}

#[test]
fn test_connected_true2(){
  let a = Edge::new(0,5,1.0);
  let b = Edge::new(5,6,1.0);

  let result = check_connected(&Rc::new(a), &Rc::new(b));

  assert_eq!(result, Some((false,true)));
}

#[test]
fn test_connected_false(){
  let a = Edge::new(0,1,1.0);
  let b = Edge::new(2,3,1.0);

  let result = check_connected(&Rc::new(a), &Rc::new(b));

  assert_eq!(result, None);
}

//------------------------------------------------------------ 


#[test]
fn test_removal_linear1(){
  let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/linear1.txt").unwrap();

  let (edges, chains) = remove_val_2_nodes(input_file);

  assert_eq!(*edges[0], Edge::new(0,3,3.0));

  assert_eq!(chains[0].start, 0);
  assert_eq!(chains[0].end, 3);
  assert_eq!(chains[0].dist, 3.0);
  assert_eq!(chains[0].nodes, vec![(0,0.0),(1,1.0),(2,2.0),(3,3.0)]);
}

#[test]
fn test_removal_linear2(){
  let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/linear2.txt").unwrap();

  let (edges, chains) = remove_val_2_nodes(input_file);

  assert_eq!(*edges[0], Edge::new(0,2,2.0));

  assert_eq!(*edges[1], Edge::new(0,3,1.0));

  assert_eq!(chains[0].start, 0);
  assert_eq!(chains[0].end, 2);
  assert_eq!(chains[0].dist, 2.0);
  assert_eq!(chains[0].nodes, vec![(0,0.0),(1,1.0),(2,2.0)]);
}

#[test]
fn test_removal_linear3(){
  let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/linear3.txt").unwrap();

  let (edges, chains) = remove_val_2_nodes(input_file);

  assert_eq!(*edges[0], Edge::new(0,2,3.0));

  assert_eq!(chains[0].start, 0);
  assert_eq!(chains[0].end, 2);
  assert_eq!(chains[0].dist, 3.0);
  assert_eq!(chains[0].nodes, vec![(0,0.0),(3,1.0),(1,2.0),(2,3.0)]);
}

#[test]
fn test_removal_cycle(){
  let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/cycle.txt").unwrap();

  let (edges, chains) = remove_val_2_nodes(input_file);

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
  let input_file = File::open("src/bin/graph_io/reduce_graph/tests/graphs/complex.txt").unwrap();

  let (edges, chains) = remove_val_2_nodes(input_file);

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
