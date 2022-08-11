use crate::helpers::tree_node::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		Self::helper(root, std::i64::MIN, std::i64::MAX)
	}

	pub fn helper(
		root: Option<Rc<RefCell<TreeNode>>>,
		min: i64,
		max: i64,
	) -> bool {
		match root {
			None => true,
			Some(root) => {
				let root = root.borrow();
				let val = root.val as i64;
				val < max
					&& val > min && Self::helper(root.left.clone(), min, val)
					&& Self::helper(root.right.clone(), val, max)
			}
		}
	}
}
