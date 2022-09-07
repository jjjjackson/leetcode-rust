use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
		match root {
			None => "".to_string(),
			Some(root) => {
				let mut root = root.borrow_mut();
				let mut result = root.val.to_string();

				if root.left.is_some() {
					result = format!(
						"{}({})",
						result,
						Self::tree2str(root.left.take())
					);
				} else if root.right.is_some() {
					result = format!("{}()", result);
				}

				if root.right.is_some() {
					result = format!(
						"{}({})",
						result,
						Self::tree2str(root.right.take())
					);
				}

				result
			}
		}
	}
}
