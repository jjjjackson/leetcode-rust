use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::collections::HashSet;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
	pub fn find_target_helper(
		root: Option<Rc<RefCell<TreeNode>>>,
		k: i32,
		set: &mut HashSet<i32>,
	) -> bool {
		root.map(|root| {
			let mut root = root.borrow_mut();
			if set.get(&root.val).is_some() {
				true
			} else {
				set.insert(k - root.val);
				Self::find_target_helper(root.left.take(), k, set)
					|| Self::find_target_helper(root.right.take(), k, set)
			}
		})
		.unwrap_or(false)
	}

	pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
		let mut set: HashSet<i32> = HashSet::new();

		Self::find_target_helper(root, k, &mut set)
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
