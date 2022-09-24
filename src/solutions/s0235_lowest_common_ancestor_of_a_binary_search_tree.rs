// Definition for a binary tree node.
use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn lowest_common_ancestor(
		root: Option<Rc<RefCell<TreeNode>>>,
		p: Option<Rc<RefCell<TreeNode>>>,
		q: Option<Rc<RefCell<TreeNode>>>,
	) -> Option<Rc<RefCell<TreeNode>>> {
		if let (Some(root), Some(p), Some(q)) = (root.as_ref(), p, q) {
			let root_val = root.borrow().val;
			let p_val = p.borrow().val;
			let q_val = q.borrow().val;
			if root_val > p_val && root_val > q_val {
				Self::lowest_common_ancestor(root.borrow().left.clone(), Some(p), Some(q))
			} else if root_val < p_val && root_val < q_val {
				Self::lowest_common_ancestor(root.borrow().right.clone(), Some(p), Some(q))
			} else {
				Some(root.clone())
			}
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::helpers::tree::TreeNode;
	use std::cell::RefCell;
	use std::ops::Deref;
	use std::rc::Rc;
	use test_case::test_case;

	#[test_case(tree![6,2,8,0,4,7,9,null,null,3,5], tree![2], tree![8], 6)]
	#[test_case(tree![6,2,8,0,4,7,9,null,null,3,5], tree![2], tree![4], 2)]
	#[test_case(tree![2,1], tree![2], tree![1   ], 2)]
	fn success_cases(
		node: Option<Rc<RefCell<TreeNode>>>,
		p: Option<Rc<RefCell<TreeNode>>>,
		q: Option<Rc<RefCell<TreeNode>>>,
		expected: i32,
	) {
		assert_eq!(
			super::Solution::lowest_common_ancestor(node, p, q)
				.unwrap()
				.deref()
				.borrow()
				.val,
			expected
		);
	}
}
