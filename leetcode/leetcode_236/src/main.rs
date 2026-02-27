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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q {
            return root;
        }
        let Some(node) = root else {
            return None;
        };

        let (left, right) = {
            let node = node.borrow();
            (node.left.clone(), node.right.clone())
        };
        let left_ancestor = Self::lowest_common_ancestor(left, p.clone(), q.clone());
        let right_ancestor = Self::lowest_common_ancestor(right, p, q);
        match (left_ancestor, right_ancestor) {
            (None, None) => None,
            (left_ancestor, None) => left_ancestor,
            (None, right_ancestor) => right_ancestor,
            (_, _) => Some(node),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
