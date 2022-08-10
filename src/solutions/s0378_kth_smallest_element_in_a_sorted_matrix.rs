// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
use crate::solutions::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Question 0378
impl Solution {
	pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
		let mut heap: BinaryHeap<Reverse<i32>> = matrix
			.into_iter()
			.flat_map(|arr| arr.into_iter().map(Reverse))
			.collect();

		(1..k).for_each(|_| {
			heap.pop();
		});

		if let Some(Reverse(value)) = heap.pop() {
			value
		} else {
			0
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;

	#[test_case(vec![vec![1,5,9],vec![10,11,13],vec![12,13,15]], 8, 13)]
	fn normal(input: Vec<Vec<i32>>, k: i32, expect: i32) {
		assert_eq!(Solution::kth_smallest(input, k), expect);
	}
}
