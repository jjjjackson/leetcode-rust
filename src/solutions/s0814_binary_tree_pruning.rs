// https://leetcode.com/problems/binary-tree-pruning/

use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		match root {
			None => None,
			Some(root) => {
				let mut root_ref = root.borrow_mut();
				root_ref.left = Self::prune_tree(root_ref.left.take());
				root_ref.right = Self::prune_tree(root_ref.right.take());

				if root_ref.left.is_none() && root_ref.right.is_none() && root_ref.val == 0 {
					None
				} else {
					drop(root_ref);
					Some(root)
				}
			}
		}
	}
}
