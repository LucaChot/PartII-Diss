pub mod reduction;
pub mod expansion;

pub struct Val2Node {
  node : usize,
  node_from : usize,
  node_from_dist : isize,
  node_to : usize,
  node_to_dist : isize,
}

impl Val2Node {
  pub fn new(node : usize, node_from : usize, node_from_dist : isize, node_to : usize, node_to_dist : isize) -> Val2Node {
    Val2Node { node, node_from, node_from_dist, node_to, node_to_dist }
  }
}

