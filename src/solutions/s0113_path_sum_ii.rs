// https://leetcode.com/problems/path-sum-ii/submissions/
use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;

// 100 %
impl Solution {
	pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
		root.map(|root| {
			let mut root = root.borrow_mut();
			if target_sum - root.val == 0 && root.left.is_none() && root.right.is_none() {
				vec![vec![root.val]]
			} else {
				let mut left = Self::path_sum(root.left.take(), target_sum - root.val);
				let mut right = Self::path_sum(root.right.take(), target_sum - root.val);

				left.append(&mut right);
				left.into_iter()
					.filter(|v| !v.is_empty())
					.map(|mut v| {
						v.insert(0, root.val);
						v
					})
					.collect::<_>()
			}
		})
		.unwrap_or_else(|| vec![])
	}
}
