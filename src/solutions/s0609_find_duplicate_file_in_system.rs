use crate::solutions::Solution;

impl Solution {
	pub fn split_filename_and_data(
		filename_and_data: String,
	) -> (String, String) {
		let chars = filename_and_data.chars().collect::<Vec<char>>();
		for i in (0..chars.len() - 1).rev() {
			if chars[i] == '(' {
				return (
					chars[0..i].iter().collect::<String>(),
					chars[i + 1..chars.len() - 1].iter().collect::<String>(),
				);
			}
		}
		return ("".to_string(), "".to_string());
	}

	pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
		use std::collections::HashMap;
		let mut map: HashMap<String, Vec<String>> = HashMap::new();

		for path in paths {
			let splited = path.split_whitespace().collect::<Vec<&str>>();
			let root = splited[0];
			let files = splited.get(1..).unwrap();

			for file in files {
				let (filename, data) =
					Self::split_filename_and_data(file.to_string());
				map.entry(data)
					.or_default()
					.push(format!("{root}/{filename}"));
			}
		}

		map.values()
			.into_iter()
			.filter(|v| v.len() > 1)
			.map(|v| v.clone())
			.collect::<Vec<Vec<String>>>()
	}
}

#[cfg(test)]
mod tests {
	use test_case::test_case;

	#[test_case(vec!["root/a 1.txt(abcd) 2.txt(efsfgh)","root/c 3.txt(abdfcd)","root/c/d 4.txt(efggdfh)"], vec![])]
	fn success_case(paths: Vec<&str>, expected: Vec<Vec<&str>>) {
		let paths =
			paths.iter().map(|s| s.to_string()).collect::<Vec<String>>();
		let result = super::Solution::find_duplicate(paths);
		assert_eq!(result, expected);
	}
}
