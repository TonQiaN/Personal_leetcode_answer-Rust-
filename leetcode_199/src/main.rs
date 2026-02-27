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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<i32>) {
            if let Some(root_node) = root {
                let (val, left, right) = {
                    let n = root_node.borrow();
                    (n.val, n.left.clone(), n.right.clone())
                };
                if depth == ans.len() {
                    ans.push(val);
                }
                dfs(&right, depth + 1, ans);
                dfs(&left, depth + 1, ans);
            }
        }
        dfs(&root, 0, &mut ans);
        ans
    }
}
// impl Solution {
//     pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//         use std::collections::VecDeque;
//         let mut ans = vec![];
//         let mut queue = VecDeque::new();
//         let Some(root_node) = root else {
//             return ans;
//         };
//         queue.push_back(root_node);
//         while !queue.is_empty() {
//             let right_side = queue.iter().last().unwrap();
//             ans.push(right_side.borrow().val);
//             for _ in 0..queue.len() {
//                 let node = queue.pop_front().unwrap();
//                 let (left, right) = {
//                     let n = node.borrow();
//                     (n.left.clone(), n.right.clone())
//                 };
//                 if left.is_some() {
//                     queue.push_back(left.unwrap());
//                 }
//                 if right.is_some() {
//                     queue.push_back(right.unwrap());
//                 }
//             }
//         }
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
