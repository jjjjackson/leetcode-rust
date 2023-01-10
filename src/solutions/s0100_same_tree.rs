struct Solution {}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn is_same_tree(
		p: Option<Rc<RefCell<TreeNode>>>,
		q: Option<Rc<RefCell<TreeNode>>>,
	) -> bool {
		match (p, q) {
			(None, None) => true,
			(Some(p), Some(q)) => {
				let mut p = p.borrow_mut();
				let mut q = q.borrow_mut();
				p.val == q.val
					&& Self::is_same_tree(p.left.take(), q.left.take())
					&& Self::is_same_tree(p.right.take(), q.right.take())
			}
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
