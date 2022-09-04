// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
use crate::helpers::tree::TreeNode;
use crate::solutions::Solution;

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
use std::collections::{BTreeMap, BinaryHeap, VecDeque};
use std::rc::Rc;

// BFS
// DFS 好想很多，解法也沒有這麼困難，可以下次再試試看
impl Solution {
	pub fn vertical_traversal(
		root: Option<Rc<RefCell<TreeNode>>>,
	) -> Vec<Vec<i32>> {
		let mut result_map = BTreeMap::new(); // BTreeMap<col<rol, value>>
		let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
		if let Some(root) = root {
			queue.push_back((root, 0));
		}

		let mut row = 0;
		while !queue.is_empty() {
			let level_len = queue.len();
			for _ in 0..level_len {
				let (node, col) = queue.pop_front().unwrap();
				result_map
					.entry(col)
					.or_insert(BTreeMap::new())
					.entry(row)
					.or_insert(BinaryHeap::new())
					.push(node.borrow().val);

				node.borrow_mut()
					.left
					.take()
					.map(|left| queue.push_back((left, col - 1)));
				node.borrow_mut()
					.right
					.take()
					.map(|right| queue.push_back((right, col + 1)));
			}
			row += 1;
		}

		result_map
			.into_iter()
			.map(|(_, v)| {
				v.into_values()
					.collect::<Vec<BinaryHeap<i32>>>()
					.into_iter()
					.map(|v| {
						let mut v = v.into_vec();
						v.sort_unstable();
						v
					})
					.flatten()
					.collect::<Vec<i32>>()
			})
			.collect::<Vec<Vec<i32>>>()
	}
}
