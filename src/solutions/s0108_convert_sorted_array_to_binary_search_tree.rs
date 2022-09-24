// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

use crate::solutions::Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
	pub val: i32,
	pub left: Option<Rc<RefCell<TreeNode>>>,
	pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		TreeNode {
			val,
			left: None,
			right: None,
		}
	}
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
	pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		if nums.is_empty() {
			None
		} else {
			let num_len: usize = nums.len();
			let half_num_len: usize = num_len / 2;
			Some(Rc::new(RefCell::new(TreeNode {
				val: nums[num_len / 2],
				left: Self::sorted_array_to_bst(nums[0..half_num_len].to_vec()),
				right: Self::sorted_array_to_bst(nums[(half_num_len + 1)..num_len].to_vec()),
			})))
		}
	}
}
