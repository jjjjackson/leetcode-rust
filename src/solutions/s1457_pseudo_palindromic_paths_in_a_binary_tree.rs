use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut count = 0;
		if let Some(root) = root.as_ref() {
			Self::pseudo_palindromic_paths_dfs(root, [0; 10], &mut count);
		}
		count
	}

	fn pseudo_palindromic_paths_dfs(
		root: &Rc<RefCell<TreeNode>>,
		freq: [i32; 10],
		count: &mut i32,
	) {
		let root_ref = root.borrow();
		let mut freq = freq;
		freq[root_ref.val as usize] += 1;

		if root_ref.left.is_none() && root_ref.right.is_none() {
			let odd_count = freq.iter().filter(|a| *a % 2 == 1).count();
			if odd_count <= 1 {
				*count += 1;
			}
		} else {
			if let Some(node) = root_ref.left.as_ref() {
				Self::pseudo_palindromic_paths_dfs(node, freq, count);
			}
			if let Some(node) = root_ref.right.as_ref() {
				Self::pseudo_palindromic_paths_dfs(node, freq, count);
			}
		}
	}
}
