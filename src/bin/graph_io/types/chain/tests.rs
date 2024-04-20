use std::fs::File;

use super::{Chain, store_chains, load_chain};

#[test]
fn load_singleton(){
  let chain_file = "src/bin/graph_io/types/chain/tests/single_chain";
  let mut a = Chain::new();
  a.start = 0;
  a.end = 3;
  a.dist = 6.0;
  a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,6.0)];

  let chains = vec![a];

  let _ = store_chains(&chains, chain_file);

  let mut file = File::open(chain_file).unwrap();
  let b = load_chain(0, &mut file);

  assert_eq!(chains[0].start, b.start);
  assert_eq!(chains[0].end, b.end);
  assert_eq!(chains[0].dist, b.dist);
  assert_eq!(chains[0].nodes, b.nodes);
}

#[test]
fn load_within_multiple(){
  let chain_file = "src/bin/graph_io/types/chain/tests/multiple_chain";
  let mut chains = Vec::new();
  for i in 1..5 {
    let mut a = Chain::new();
    a.start = 0;
    a.end = i;
    a.dist = 2.0 * i as f64;
    a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,a.dist)];
    chains.push(a);
  }

  let _ = store_chains(&chains, chain_file);

  let mut file = File::open(chain_file).unwrap();
  let b = load_chain(2, &mut file);

  assert_eq!(chains[2].start, b.start);
  assert_eq!(chains[2].end, b.end);
  assert_eq!(chains[2].dist, b.dist);
  assert_eq!(chains[2].nodes, b.nodes);
}

#[test]
fn load_last(){
  let chain_file = "src/bin/graph_io/types/chain/tests/last_chain";
  let mut chains = Vec::new();
  for i in 1..5 {
    let mut a = Chain::new();
    a.start = 0;
    a.end = i;
    a.dist = 2.0 * i as f64;
    a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,a.dist)];
    chains.push(a);
  }

  let _ = store_chains(&chains, chain_file);

  let mut file = File::open(chain_file).unwrap();
  let b = load_chain(3, &mut file);

  assert_eq!(chains[3].start, b.start);
  assert_eq!(chains[3].end, b.end);
  assert_eq!(chains[3].dist, b.dist);
  assert_eq!(chains[3].nodes, b.nodes);
}

#[test]
fn test_get_entire_path(){
  let mut a = Chain::new();
  a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,6.0), (4,10.0)];

  assert_eq!(a.get_path(0, 4), vec![0,1,2,3,4]);
}

#[test]
fn test_get_entire_path_reversed(){
  let mut a = Chain::new();
  a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,6.0), (4,10.0)];

  assert_eq!(a.get_path(4, 0), vec![4,3,2,1,0]);
}

#[test]
fn test_get_slice_path(){
  let mut a = Chain::new();
  a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,6.0), (4,10.0)];

  assert_eq!(a.get_path(1, 3), vec![1,2,3]);
}

#[test]
fn test_get_slice_path_reversed(){
  let mut a = Chain::new();
  a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,a.dist)];

  assert_eq!(a.get_path(3, 1), vec![3,2,1]);
}

#[test]
fn test_get_singleton_path(){
  let mut a = Chain::new();
  a.nodes = vec![(0,0.0), (1,1.0), (2,3.0), (3,a.dist)];

  assert_eq!(a.get_path(1, 1), vec![1]);
}
