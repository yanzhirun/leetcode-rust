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
use std::collections::VecDeque;
use std::rc::Rc;
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    let mut vq = VecDeque::new();
    if let Some(node) = root {
        vq.push_back(node);
    }

    while !vq.is_empty() {
        ans.push(vq.back().unwrap().borrow().val);
        for _ in 0..vq.len() {
            if let Some(node) = vq.pop_front() {
                if let Some(left) = node.borrow_mut().left.take() {
                    vq.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    vq.push_back(right);
                }
            }
        }
    }

    ans
}

#[cfg(test)]
#[test]
pub fn test_199() {
    let mut root = TreeNode::new(1);
    let mut l21 = TreeNode::new(2);
    let mut l22 = TreeNode::new(3);
    let l31 = TreeNode::new(5);
    let l32 = TreeNode::new(4);
    l21.right = Some(Rc::new(RefCell::new(l31)));
    l22.right = Some(Rc::new(RefCell::new(l32)));

    root.left = Some(Rc::new(RefCell::new(l21)));
    root.right = Some(Rc::new(RefCell::new(l22)));

    let input = Some(Rc::new(RefCell::new(root)));
    let test_case_res1 = vec![1, 3, 4];

    assert_eq!(test_case_res1, right_side_view(input));
}
