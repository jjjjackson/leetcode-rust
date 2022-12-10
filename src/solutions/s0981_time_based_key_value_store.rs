use std::collections::HashMap;

struct TimeMap {
	store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
	fn new() -> Self {
		TimeMap {
			store: HashMap::new(),
		}
	}

	fn set(&mut self, key: String, value: String, timestamp: i32) {
		let entry = self.store.entry(key).or_insert(Vec::new());
		entry.push((timestamp, value));
	}

	fn get(&self, key: String, timestamp: i32) -> String {
		if let Some(timestamp_value) = self.store.get(&key) {
			match timestamp_value.binary_search_by_key(&timestamp, |&(t, _)| t) {
				Ok(i) => timestamp_value[i].1.clone(),
				Err(i) if i > 0 => timestamp_value[i - 1].1.clone(),
				_ => "".to_string(),
			}
		} else {
			"".to_string()
		}
	}
}
