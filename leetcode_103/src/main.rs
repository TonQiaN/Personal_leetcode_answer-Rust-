// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
struct Solution {}
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut ans = vec![];
        let Some(root_node) = root else {return ans;};
        let mut order = true;
        queue.push_back(root_node); 
        while !queue.is_empty() {
            let layer_size = queue.len();
            let mut layer_values = vec![];
            for _ in 0..layer_size {
                if let Some(node) = queue.pop_front(){
                    let node_borrow = node.borrow();
                    layer_values.push(node_borrow.val);
                    if let Some(l) = node_borrow.left.clone() {queue.push_back(l);};
                    if let Some(r) = node_borrow.right.clone() {queue.push_back(r);};
                }
            }
            if !order {
                layer_values.reverse();
            }
            order = !order;
            ans.push(layer_values)
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
