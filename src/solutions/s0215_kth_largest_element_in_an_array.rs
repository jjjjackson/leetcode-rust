// https://leetcode.com/problems/kth-largest-element-in-an-array/
use crate::solutions::Solution;
use std::collections::BinaryHeap;

impl Solution {
	pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
		let mut heap = BinaryHeap::new();
		nums.into_iter().for_each(|i| {
			heap.push(i);
		});

		for _ in 0..k - 1 {
			heap.pop();
		}

		heap.pop().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case(vec![3,2,1,5,6,4], 2, 5)]
	#[test_case(vec![3,2,3,1,2,4,5,5,6], 4, 4)]
	fn normal(input: Vec<i32>, k: i32, expect: i32) {
		assert_eq!(Solution::find_kth_largest(input, k), expect);
	}
}
