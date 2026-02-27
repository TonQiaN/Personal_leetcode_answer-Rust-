#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}
struct Solution{

}

impl Solution {
    fn new() -> Self {
        Solution{}
    }

    pub fn isCompleteTree(&self, root: Option<Box<TreeNode>>) -> bool {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut meet_none = false;
        queue.push_back(&root);
        while let Some(root) = queue.pop_front() {
            if let Some(node) = root {
                if meet_none {
                    return false;
                }
                queue.push_back(&node.left);
                queue.push_back(&node.right);
            } else {
                meet_none = true;
            }
        }
        true
    }
}