use crate::solutions::Solution;
// use std::collections::HashMap;

// 12.37%
// impl Solution {
//     pub fn first_uniq_char(s: String) -> i32 {
//         let mut counter: HashMap<char, i32> = HashMap::new();
//         for c in s.chars() {
//             let new_num = counter.get(&c).unwrap_or_else(|| &0) + 1;
//             counter.insert(c, new_num);
//         }

//         for (i,c) in s.chars().enumerate() {
//             if counter.get(&c).unwrap_or_else(|| &0) == &1 {
//                 return i as i32;
//             }
//         }

//         -1
//     }
// }

// 89.69%
impl Solution {
	pub fn first_uniq_char(s: String) -> i32 {
		let mut counter = vec![0i32; 26];
		let a_byte = 'a'.to_string().as_bytes()[0];
		for c in s.as_bytes() {
			counter[(c - a_byte) as usize] += 1;
		}

		for (i, c) in s.as_bytes().iter().enumerate() {
			if counter[(c - a_byte) as usize] == 1 {
				return i as i32;
			}
		}

		-1
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("aabb", -1)]
	#[test_case("leetcode", 0)]
	fn success_case(s: &str, expected: i32) {
		assert_eq!(super::Solution::first_uniq_char(s.to_string()), expected);
	}
}
