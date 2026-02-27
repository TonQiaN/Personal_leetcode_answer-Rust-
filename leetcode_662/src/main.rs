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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let mut ans = 0i64;
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i64)> = VecDeque::new();
        let Some(root_node) = root else {return ans as i32;};
        queue.push_back((root_node, 1));
        while !queue.is_empty() {
            let layer_size = queue.len();
            let mut local_min = i64::MAX;
            let mut local_max = i64::MIN;
            for _ in 0..layer_size {
                let (node, i) = queue.pop_front().unwrap();
                let node_borrow = node.borrow();
                local_min = local_min.min(i);
                local_max = local_max.max(i);
                if let Some(l) = node_borrow.left.clone() {queue.push_back((l, i * 2));};
                if let Some(r) = node_borrow.right.clone() {queue.push_back((r, i * 2 + 1));};
            }
            ans = ans.max(local_max - local_min + 1);
        }
        ans as i32
    }
}

fn main() {
    println!("Hello, world!");
}
