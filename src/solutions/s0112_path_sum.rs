use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
		root.map(|root| {
			let mut root = root.borrow_mut();
			if root.left.is_none() && root.right.is_none() && root.val == target_sum {
				true
			} else {
				Self::has_path_sum(root.left.take(), target_sum - root.val)
					|| Self::has_path_sum(root.right.take(), target_sum - root.val)
			}
		})
		.unwrap_or(false)
	}
}
