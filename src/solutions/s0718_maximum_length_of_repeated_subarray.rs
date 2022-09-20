use crate::solutions::Solution;

impl Solution {
	pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
		let mut result = 0;
		let table = (0..nums1.len())
			.map(|i| (i, 0))
			.chain((1..nums2.len()).map(|j| (0, j))); // 所有的對角線

		for (i, j) in table {
			let mut iter1 = nums1[i..].iter();
			let mut iter2 = nums2[j..].iter();
			let mut dp = 0;

			while let (Some(&x), Some(&y)) = (iter1.next(), iter2.next()) {
				if x == y {
					dp += 1;
					result = result.max(dp);
				} else {
					dp = 0;
				}
			}
		}
		result
	}
}

// 自己寫的，會 Timeout
// impl Solution {
// 	pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
// 		let is_included = |nums1: &[i32], nums2: &Vec<i32>| -> bool {
// 			let keys = nums2.iter().enumerate().filter_map(|(key, value)| {
// 				if *value == nums1[0] {
// 					Some(key)
// 				} else {
// 					None
// 				}
// 			})
// 			.collect::<Vec<usize>>();

// 			for key in keys {
// 				let mut is_same = true;
// 				for i in 0..nums1.len() {
// 					if key + i >= nums2.len() || nums2[key + i] != nums1[i]
// 					{
// 						is_same = false;
// 					}
// 				}
// 				if is_same {
// 					return true;
// 				}
// 			}

// 			false
// 		};

// 		let common_len = nums1.len().min(nums2.len());

// 		for len in (0..common_len).rev() {
// 			let mut i = 0;
// 			while i + len < nums1.len() {
// 				let chunk = nums1.get(i..(i + len + 1)).unwrap();
// 				if is_included(chunk, &nums2) {
// 					return (len + 1) as i32;
// 				}
// 				i += 1;
// 			}
// 		}

// 		0
// 	}
// }

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![1,2,3,2,1], vec![3,2,1,4,7], 3)]
	#[test_case(vec![70,39,25,40,7],vec![52,20,67,5,31],0)]
	#[test_case(vec![5,14,53,80,48],vec![50,47,3,80,83],1)]
	#[test_case(vec![0,0,0,0,0,0,1,0,0,0], vec![0,0,0,0,0,0,0,1,0,0], 9)]
	fn success_case(nums1: Vec<i32>, nums2: Vec<i32>, expected: i32) {
		assert_eq!(super::Solution::find_length(nums1, nums2), expected);
	}
}
