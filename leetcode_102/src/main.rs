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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        let mut ans = vec![];
        let Some(root_node) = root else {
            return ans;
        };
        queue.push_back(root_node.clone());
        while !queue.is_empty() {
            let n = queue.len();
            let mut layer_int = vec![];
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let (value, left, right) = {
                    let n = node.borrow();
                    (n.val, n.left.clone(), n.right.clone())
                };
                layer_int.push(value);
                if let Some(l) = left {
                    queue.push_back(l);
                };
                if let Some(r) = right {
                    queue.push_back(r);
                };
            }
            ans.push(layer_int);
        }
        ans
    }
}
// use std::rc::Rc;
// use std::cell::RefCell;
// impl Solution {
//     pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
//         use std::collections::VecDeque;
//         let mut queue = VecDeque::new();
//         let mut ans = vec![];
//         let Some(node) = root else {return ans;};
//         queue.push_back(node);
//         while !queue.is_empty() {
//             let layer_size = queue.len();
//             let mut layer_values = Vec::with_capacity(layer_size);
//             for _ in 0..layer_size {
//                 let node = queue.pop_front().unwrap();
//                 let node_borrow = node.borrow();
//                 layer_values.push(node_borrow.val);
//                 if let Some(l) = node_borrow.left.clone() {queue.push_back(l);}
//                 if let Some(r) = node_borrow.right.clone() {queue.push_back(r);}
//             }
//             ans.push(layer_values);
//         }
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
