use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		Self::helper(root, std::i64::MIN, std::i64::MAX)
	}

	pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
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

#[cfg(test)]
mod tests {
	use test_case::test_case;

	use crate::helpers::tree::TreeNode;
	use std::cell::RefCell;
	use std::rc::Rc;

	#[test_case(tree![5, 1, 4, null, null, 3, 6], false)]
	#[test_case(tree![2, 1, 3], true)]
	#[test_case(tree![10, 5, 15, null, null, 6, 20], false)]
	#[test_case(tree![2147483647], true)]
	#[test_case(tree![5,4,6,null,null,3,7], false)]
	#[test_case(tree![0], true)]
	#[test_case(tree![1,1,1], false)]
	#[test_case(tree![1,null,1], false)]
	#[test_case(tree![1,1,null], false)]
	fn success_cases(node: Option<Rc<RefCell<TreeNode>>>, expected: bool) {
		assert_eq!(super::Solution::is_valid_bst(node), expected);
	}
}
