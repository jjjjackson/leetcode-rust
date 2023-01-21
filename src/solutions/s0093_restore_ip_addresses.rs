struct Solution {}

impl Solution {
	pub fn restore_ip_addresses(s: String) -> Vec<String> {
		let mut answers: Vec<String> = Vec::new();
		let s_vec = s.chars().collect::<Vec<char>>();
		Solution::helper(&s_vec, 0, 0, "".to_string(), &mut answers);
		answers
	}

	pub fn helper(
		s: &Vec<char>,
		index: usize,
		dot_size: usize,
		path: String,
		answers: &mut Vec<String>,
	) {
		if dot_size > 4 {
			return;
		} else if dot_size == 4 && index >= s.len() {
			answers.push(path[0..path.len() - 1].to_string());
		} else {
			(1..=3).filter(|i| index + i <= s.len()).for_each(|i| {
				let number_str = &s[index..index + i];
				let number: i32 = number_str
					.iter()
					.collect::<String>()
					.parse::<i32>()
					.unwrap();
				if (number_str[0] != '0' || number_str.len() == 1) && number < 256 {
					Solution::helper(
						s,
						index + number_str.len(),
						dot_size + 1,
						format!("{path}{}.", number_str.iter().collect::<String>()),
						answers,
					);
				}
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case("25525511135",vec!["255.255.11.135","255.255.111.35"])]
	fn cases(s: &str, expected: Vec<&str>) {
		let result = super::Solution::restore_ip_addresses(s.to_string());
		println!("{result:?}");
		assert!(expected.iter().all(|a| result.contains(&a.to_string())));
	}
}
