use std::fs::File;

use crate::types::node::ReducedState;

use super::{store_states, load_state, GraphState, ChainState};

#[test]
fn load_singleton_ingraph(){
  let state_file = "src/bin/graph_io/types/node/tests/single_ingraph";
  let mut graph_state = GraphState::new(3);
  graph_state.chains = vec![1,2,3];
  let a = ReducedState::INGRAPH(graph_state);

  let states = vec![&a];

  let _ = store_states(&states, state_file);

  let mut file = File::open(state_file).unwrap();
  let b = load_state(0, &mut file);

  match b {
    ReducedState::INGRAPH(g) => {
      assert_eq!(3, g.reduced_id);
      assert_eq!(vec![1,2,3], g.chains);
    },
    _ => panic!()
  }
}

#[test]
fn load_singleton_inchain(){
  let state_file = "src/bin/graph_io/types/node/tests/single_inchain";
  let chain_state = ChainState::new(1,2,3.0,4,5.0);
  let a = ReducedState::INCHAIN(chain_state);

  let states = vec![&a];

  let _ = store_states(&states, state_file);

  let mut file = File::open(state_file).unwrap();
  let b = load_state(0, &mut file);

  match b {
    ReducedState::INCHAIN(c) => {
      assert_eq!(1, c.chain_id);
      assert_eq!(2, c.end_a);
      assert_eq!(3.0, c.dist_a);
      assert_eq!(4, c.end_b);
      assert_eq!(5.0, c.dist_b);
    },
    _ => panic!()
  }
}

#[test]
fn load_within_multiple_ingraph(){
  let state_file = "src/bin/graph_io/types/node/tests/multiple_state";

  let mut graph_state1 = GraphState::new(3);
  graph_state1.chains = vec![1,2,3];
  let a = ReducedState::INGRAPH(graph_state1);

  let chain_state2 = ChainState::new(1,2,3.0,4,5.0);
  let b = ReducedState::INCHAIN(chain_state2);

  let mut graph_state3 = GraphState::new(2);
  graph_state3.chains = vec![3,2,1];
  let c = ReducedState::INGRAPH(graph_state3);

  let mut graph_state4 = GraphState::new(8);
  graph_state4.chains = vec![2,4,6];
  let d = ReducedState::INGRAPH(graph_state4);

  let states = vec![&a, &b, &c, &d];

  let _ = store_states(&states, state_file);

  let mut file = File::open(state_file).unwrap();
  let r1 = load_state(2, &mut file);

  match r1 {
    ReducedState::INGRAPH(g) => {
      assert_eq!(2, g.reduced_id);
      assert_eq!(vec![3,2,1], g.chains);
    },
    _ => panic!()
  }

  let mut file = File::open(state_file).unwrap();
  let r2 = load_state(1, &mut file);

  match r2 {
    ReducedState::INCHAIN(c) => {
      assert_eq!(1, c.chain_id);
      assert_eq!(2, c.end_a);
      assert_eq!(3.0, c.dist_a);
      assert_eq!(4, c.end_b);
      assert_eq!(5.0, c.dist_b);
    },
    _ => panic!()
  }
}

#[test]
fn load_last_ingraph(){
  let state_file = "src/bin/graph_io/types/node/tests/last_ingraph";

  let mut graph_state1 = GraphState::new(3);
  graph_state1.chains = vec![1,2,3];
  let a = ReducedState::INGRAPH(graph_state1);

  let chain_state2 = ChainState::new(1,2,3.0,4,5.0);
  let b = ReducedState::INCHAIN(chain_state2);

  let mut graph_state3 = GraphState::new(2);
  graph_state3.chains = vec![3,2,1];
  let c = ReducedState::INGRAPH(graph_state3);

  let mut graph_state4 = GraphState::new(8);
  graph_state4.chains = vec![2,4,6];
  let d = ReducedState::INGRAPH(graph_state4);

  let states = vec![&a, &b, &c, &d];

  let _ = store_states(&states, state_file);

  let mut file = File::open(state_file).unwrap();
  let b = load_state(3, &mut file);

  match b {
    ReducedState::INGRAPH(g) => {
      assert_eq!(8, g.reduced_id);
      assert_eq!(vec![2,4,6], g.chains);
    },
    _ => panic!()
  }
}

#[test]
fn load_last_inchain(){
  let state_file = "src/bin/graph_io/types/node/tests/last_inchain";

  let mut graph_state1 = GraphState::new(3);
  graph_state1.chains = vec![1,2,3];
  let a = ReducedState::INGRAPH(graph_state1);

  let chain_state2 = ChainState::new(1,2,3.0,4,5.0);
  let b = ReducedState::INCHAIN(chain_state2);

  let mut graph_state3 = GraphState::new(2);
  graph_state3.chains = vec![3,2,1];
  let c = ReducedState::INGRAPH(graph_state3);

  let chain_state4 = ChainState::new(2,4,6.0,8,10.0);
  let d = ReducedState::INCHAIN(chain_state4);

  let states = vec![&a, &b, &c, &d];

  let _ = store_states(&states, state_file);

  let mut file = File::open(state_file).unwrap();
  let r = load_state(3, &mut file);

  match r {
    ReducedState::INCHAIN(c) => {
      assert_eq!(2, c.chain_id);
      assert_eq!(4, c.end_a);
      assert_eq!(6.0, c.dist_a);
      assert_eq!(8, c.end_b);
      assert_eq!(10.0, c.dist_b);
    },
    _ => panic!()
  }
}
