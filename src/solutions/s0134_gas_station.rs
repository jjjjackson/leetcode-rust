struct Solution {}

impl Solution {
	pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
		let mut current_gas = 0;
		let (mut index, mut min) = (0, 0);
		for i in 0..gas.len() {
			current_gas += gas[i] - cost[i];
			if current_gas < min {
				min = current_gas;
				index = i + 1;
			}
		}
		if current_gas < 0 {
			-1
		} else {
			index as i32
		}
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![1,2,3,4,5], vec![3,4,5,1,2], 3)]
	fn cases(gas: Vec<i32>, cost: Vec<i32>, expected: i32) {
		assert_eq!(super::Solution::can_complete_circuit(gas, cost), expected);
	}
}
