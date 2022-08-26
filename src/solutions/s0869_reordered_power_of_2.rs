// https://leetcode.com/problems/reordered-power-of-2/submissions/
use crate::solutions::Solution;

// 3ms 50%
use std::collections::HashSet;
impl Solution {
	pub fn reordered_power_of2(n: i32) -> bool {
		let pow_set: HashSet<Vec<char>> = (0..)
			.map_while(|n| 2_i32.checked_pow(n))
			.map(|n| {
				let mut s = n.to_string().chars().collect::<Vec<char>>();
				s.sort_unstable();
				s
			})
			.collect::<_>();
		let mut n_vec = n.to_string().chars().collect::<Vec<char>>();
		n_vec.sort_unstable();
		pow_set.contains(&n_vec)
	}
}

// 6ms 50%
// use std::collections::HashMap;
// impl Solution {
// 	pub fn reordered_power_of2(n: i32) -> bool {
// 		let digit_of_pow = (0..65)
// 			.map(|i| {
// 				let pow_result = 2_u128.pow(i);

// 				let mut m: HashMap<u32, i32> = HashMap::new();
// 				pow_result
// 					.to_string()
// 					.chars()
// 					.map(|d| d.to_digit(10).unwrap_or(0))
// 					.for_each(|v| *m.entry(v).or_default() += 1);
// 				(i, m)
// 			})
// 			.collect::<HashMap<u32, HashMap<u32, i32>>>();

// 		let mut digit_of_n: HashMap<u32, i32> = HashMap::new();
// 		n.to_string()
// 			.chars()
// 			.map(|d| d.to_digit(10).unwrap_or(0))
// 			.for_each(|v| *digit_of_n.entry(v).or_default() += 1);

// 		digit_of_pow.iter().any(|(_, d)| d == &digit_of_n)
// 	}
// }

// 0ms 100%
// impl Solution {
// 	pub fn reordered_power_of2(n: i32) -> bool {
// 		let mut digit_of_pow = vec![vec![0; 10]; 65];
// 		let mut digit_of_n = vec![0; 10];

// 		digit_of_pow[0][1] = 1;
// 		digit_of_pow[1][2] = 1;
// 		let mut pow_result: u64 = 2;
// 		for i in 3..65 {
// 			pow_result *= 2;

// 			pow_result
// 				.to_string()
// 				.chars()
// 				.map(|d| d.to_digit(10).unwrap_or(0) as usize)
// 				.for_each(|c| {
// 					digit_of_pow[i as usize][c] += 1;
// 				});
// 		}

// 		n.to_string()
// 			.chars()
// 			.map(|d| d.to_digit(10).unwrap_or(0) as usize)
// 			.for_each(|c| {
// 				digit_of_n[c] += 1;
// 			});

// 		digit_of_pow.iter().any(|d| d == &digit_of_n)
// 	}
// }

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(1, true)]
	#[test_case(10, false)]
	#[test_case(32, true)]
	#[test_case(23, true)]
	#[test_case(2041, true)]
	fn success_case(n: i32, expected: bool) {
		assert_eq!(super::Solution::reordered_power_of2(n), expected);
	}
}
