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

use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, output: &mut Vec<i32>) -> Vec<i32> {
        match root {
			None => vec![],
			Some(root) => {
				Self::inorder(root.borrow().left.clone(), output);
                output.push(root.borrow().val.clone());
				Self::inorder(root.borrow().right.clone(), output);
                output.to_vec()
			}
		}
    }

	pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut result: Vec<i32> = vec![];
        Self::inorder(root, &mut result)
	}
}
