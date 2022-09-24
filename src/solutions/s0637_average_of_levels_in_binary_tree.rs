// https://leetcode.com/problems/average-of-levels-in-binary-tree/
use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
	pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
		match root {
			None => vec![],
			Some(root) => {
				let mut result: Vec<f64> = Vec::new();
				let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
				queue.push_back(root);

				while !queue.is_empty() {
					let level_len = queue.len();
					let mut total = 0_f64;

					for _ in 0..level_len {
						queue.pop_front().map(|n| {
							total += n.borrow().val as f64;
							n.borrow_mut().left.take().map(|left| {
								queue.push_back(left);
							});
							n.borrow_mut().right.take().map(|right| {
								queue.push_back(right);
							});
						});
					}

					result.push(total / level_len as f64);
				}

				result
			}
		}
	}
}
