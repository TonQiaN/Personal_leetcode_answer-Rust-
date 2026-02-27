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
use std::cell::{Ref, RefCell};
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans = vec![];
        let mut path: Vec<i32> = vec![];
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, ans: &mut Vec<String>) {
            let Some(node) = node else {
                return;
            };
            let (val, node_left, node_right) = {
                let n = node.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            path.push(val);
            if node_left.is_none() && node_right.is_none() {
                let path_format = path.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join("->");
                ans.push(path_format);
            } else {
                dfs(node_left, path, ans);
                dfs(node_right, path, ans);
            }
            path.pop();
        }
        dfs(root, &mut path, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
