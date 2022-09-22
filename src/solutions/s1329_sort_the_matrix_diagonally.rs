// https://leetcode.com/problems/sort-the-matrix-diagonally/
use crate::solutions::Solution;

impl Solution {
	pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		use std::cmp::Reverse;
		use std::collections::{BinaryHeap, HashMap};

		if mat.is_empty() {
			return mat;
		}

		let (rows, cols) = (mat.len(), mat[0].len());
		let mut mat_map: HashMap<i32, BinaryHeap<_>> = HashMap::new();
		let mut result = vec![vec![i32::MIN; cols]; rows];

		for r in 0..rows {
			for c in 0..cols {
				let key = r as i32 - c as i32;
				mat_map
					.entry(key)
					.or_insert(BinaryHeap::new())
					.push(Reverse(mat[r][c]))
			}
		}

		for (r, row) in result.iter_mut().enumerate().take(rows) {
			for c in 0..cols {
				let key = r as i32 - c as i32;
				let queue = mat_map.get_mut(&key).unwrap();
				if let Some(Reverse(value)) = queue.pop() {
					row[c] = value;
				}
			}
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![vec![3,3,1,1],vec![2,2,1,2],vec![1,1,1,2]],vec![vec![1,1,1,1],vec![1,2,2,2],vec![1,2,3,3]];"first")]
	#[test_case(vec![vec![11,25,66,1,69,7],vec![23,55,17,45,15,52],vec![75,31,36,44,58,8],vec![22,27,33,25,68,4],vec![84,28,14,11,5,50]], vec![vec![5,17,4,1,52,7],vec![11,11,25,45,8,69],vec![14,23,25,44,58,15],vec![22,27,31,36,50,66],vec![84,28,75,33,55,68]];"second")]
	fn success_case(mat: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
		assert_eq!(super::Solution::diagonal_sort(mat), expected);
	}
}
