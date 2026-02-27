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

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    pub fn inorderTraversal(&self, root: Option<Box<TreeNode>>) -> Vec<i32> {
        let mut ans = vec![];
        fn inorder(root: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
            if let Some(node) = root {  
                let (val, left, right) = (node.val, &node.left, &node.right);
                inorder(&left, ans);
                ans.push(val);
                inorder(&right, ans);
            }
        }
        inorder(&root, &mut ans);
        ans
    }
}
