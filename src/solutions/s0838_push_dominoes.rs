use crate::solutions::Solution;

impl Solution {
	pub fn push_dominoes(dominoes: String) -> String {
		let d = format!("L{}R", dominoes.clone());
		let d = d.as_bytes();
		let mut result = vec![];
		let (dot, left, right) = ('.' as u8, 'L' as u8, 'R' as u8);

		let mut i = 0;
		for j in 1..d.len() {
			if d[j] == dot {
				continue;
			}
			let mid = j - i - 1;
			if i > 0 {
				result.push(d[i]);
			}
			if d[i] == d[j] {
				(0..mid).for_each(|_| result.push(d[i]));
			} else if d[i] == left && d[j] == right {
				(0..mid).for_each(|_| result.push(dot));
			} else {
				(0..(mid / 2)).for_each(|_| result.push(right));
				if mid % 2 == 1 {
					result.push(dot);
				}
				(0..(mid / 2)).for_each(|_| result.push(left));
			}
			i = j;
		}

		result.iter().map(|c| *c as char).collect::<String>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("...L", "LLLL"; "1")]
	#[test_case("R...", "RRRR"; "2")]
	#[test_case("RR.L", "RR.L"; "3")]
	#[test_case("R..L", "RRLL"; "4")]
	#[test_case(".L.R...LR..L..", "LL.RR.LLRRLL..";"5")]
	#[test_case(".L.R.", "LL.RR"; "6")]
	#[test_case("L.....RR.RL.....L.R.", "L.....RRRRLLLLLLL.RR"; "7")]
	fn cases(dominoes: &str, expected: &str) {
		assert_eq!(
			super::Solution::push_dominoes(dominoes.to_string()),
			expected.to_string()
		);
	}
}
