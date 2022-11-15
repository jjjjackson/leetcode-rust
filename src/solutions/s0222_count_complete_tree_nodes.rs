use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		match root {
			None => 0,
			Some(root) => {
				let mut root = root.borrow_mut();
				Self::count_nodes(root.right.take()) + Self::count_nodes(root.left.take()) + 1
			}
		}
	}
}
