// https://leetcode.com/problems/two-sum/

use crate::solutions::Solution;

impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let nums_len = nums.len();
		for i in 0..nums_len {
			for j in (i + 1)..nums_len {
				if nums[i] + nums[j] == target {
					return vec![i as i32, j as i32];
				}
			}
		}

		return vec![];
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![2,7,11,15], 9, vec![0, 1])]
	#[test_case(vec![3,2,4], 6, vec![1, 2])]
	#[test_case(vec![3,3], 6, vec![0,1])]
	fn success(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
		assert_eq!(super::Solution::two_sum(nums, target), expected);
	}
}
