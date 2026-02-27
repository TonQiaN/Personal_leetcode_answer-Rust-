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

enum State {
    NotCover,
    Cover,
    Camera,
}

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut dummy_node = TreeNode::new(0);
        dummy_node.left = root;
        fn travel(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> State {
            if let Some(node) = root {
                let (left, right) = {
                    let node = node.borrow();
                    (node.left.clone(), node.right.clone())
                };
                let left_state = travel(left, ans);
                let right_state = travel(right, ans);
                match (left_state, right_state) {
                    (State::Cover, State::Cover) => State::NotCover,
                    (State::NotCover, _) | (_, State::NotCover) => {
                        *ans += 1;
                        State::Camera
                    }
                    (State::Camera, _) | (_, State::Camera) => State::Cover,
                }
            } else {
                State::Cover
            }
        }
        travel(Some(Rc::new(RefCell::new(dummy_node))), &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
