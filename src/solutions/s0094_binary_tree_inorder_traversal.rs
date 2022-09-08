use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		root.map(|root| {
			let mut root = root.borrow_mut();
			[
				Self::inorder_traversal(root.left.take()),
				vec![root.val],
				Self::inorder_traversal(root.right.take()),
			]
			.concat()
		})
		.unwrap_or_default()
	}
}

// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
// 	pub fn inorder(
// 		root: Option<Rc<RefCell<TreeNode>>>,
// 		output: &mut Vec<i32>,
// 	) -> Vec<i32> {
// 		match root {
// 			None => vec![],
// 			Some(root) => {
// 				Self::inorder(root.borrow().left.clone(), output);
// 				output.push(root.borrow().val);
// 				Self::inorder(root.borrow().right.clone(), output);
// 				output.to_vec()
// 			}
// 		}
// 	}

// 	pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
// 		let mut result: Vec<i32> = vec![];
// 		Self::inorder(root, &mut result)
// 	}
// }
