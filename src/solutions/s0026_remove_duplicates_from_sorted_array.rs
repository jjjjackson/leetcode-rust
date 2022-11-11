use crate::solutions::Solution;

impl Solution {
	pub fn remove_duplicates_0026(nums: &mut Vec<i32>) -> i32 {
		if nums.is_empty() {
			return 0;
		}

		let mut prev = 0;
		for i in 1..nums.len() {
			if nums[prev] != nums[i] {
				prev += 1;
				nums[prev] = nums[i];
			}
		}

		(prev + 1) as i32
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
