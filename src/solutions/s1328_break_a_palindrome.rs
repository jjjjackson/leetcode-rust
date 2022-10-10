use crate::solutions::Solution;

impl Solution {
	pub fn break_palindrome(palindrome: String) -> String {
		if palindrome.len() < 2 {
			return "".to_string();
		}

		let mid = palindrome.len() / 2;
		let mut bytes = palindrome.as_bytes().to_vec();

		(0..mid)
			.into_iter()
			.find(|&i| bytes[i] != 97)
			.and_then(|i| {
				bytes[i] = 97;
				Some(())
			})
			.or_else(|| {
				bytes[palindrome.len() - 1] = 98;
				Some(())
			});

		bytes.iter().map(|&byte| byte as char).collect::<String>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
