use crate::helpers::tree::TreeNode;
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
	pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		root.map_or_else(
			|| Vec::new(),
			|root| {
				let mut queue = VecDeque::from([root]);
				let mut result = vec![];

				while !queue.is_empty() {
					let node = queue.pop_back().unwrap();
					result.push(node.borrow_mut().val);

					node.borrow_mut()
						.right // 注意這個地方的順序
						.take()
						.map(|n| queue.push_back(n));

					node.borrow_mut().left.take().map(|n| queue.push_back(n));
				}

				result
			},
		)
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
