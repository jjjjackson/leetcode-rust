// 需要重新再看一下
use crate::solutions::Solution;

impl Solution {
	pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
		let mut nums = nums;
		nums.sort();

		let mut closest = nums[0] + nums[1] + nums[nums.len() - 1];

		for i in 0..(nums.len() - 2) {
			let (mut left, mut right) = (i + 1, nums.len() - 1);

			while left < right {
				let sum = nums[i] + nums[left] + nums[right];

				if sum == target {
					return sum;
				}

				if (sum - target).abs() < (closest - target).abs() {
					closest = sum;
				}

				if sum < target {
					left += 1;
				} else {
					right -= 1;
				}
			}
		}

		closest
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
