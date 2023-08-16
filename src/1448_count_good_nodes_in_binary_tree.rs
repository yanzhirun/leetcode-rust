use std::cell::RefCell;
use std::rc::Rc;
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

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, high: i32) -> i32 {
        if let Some(node) = root {
            let high = i32::max(node.borrow().val, high);
            (node.borrow().val == high) as i32
                + dfs(node.borrow().left.as_ref(), high)
                + dfs(node.borrow().right.as_ref(), high)
        } else {
            0
        }
    }
    dfs(root.as_ref(), i32::MIN)
}

#[cfg(test)]
#[test]
pub fn test_1448() {
    let input = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    assert_eq!(1, good_nodes(input));
}
