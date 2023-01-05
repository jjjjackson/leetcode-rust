struct Solution {}

impl Solution {
	pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
		if points.is_empty() {
			return 0;
		}

		points.sort_unstable_by_key(|a| a[0]);

		let mut result = 0;
		let mut range = points[0][1];

		for i in 0..points.len() {
			let point = points[i].clone();

			if point[0] > range {
				range = point[1];
				result += 1;
			} else {
				range = range.min(point[1]);
			}
		}
		(result + 1) as i32
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec![[10,16],[2,8],[1,6],[7,12]], 2)]
	fn cases(points: Vec<[i32; 2]>, expected: i32) {
		assert_eq!(
			super::Solution::find_min_arrow_shots(
				points.iter().map(|a| a.to_vec()).collect::<Vec<_>>()
			),
			expected
		);
	}
}
