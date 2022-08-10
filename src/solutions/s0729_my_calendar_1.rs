// struct MyCalendar {
//     records: Vec<(i32,i32)>
// }

// 0729
// Brute Force
//impl MyCalendar {

//     fn new() -> Self {
//         Self {
//             records: vec![]
//         }
//     }

//     fn book(&mut self, start: i32, end: i32) -> bool {
//         if self.records.iter().find(|(s,e)| *s < end && *e > start).is_some() {
//             false
//         } else {
//             self.records.push((start, end));
//             true
//         }
//     }
// }

// 0729
// BinaryTree, but only 83.33% 33ms
#[allow(dead_code)]
struct BinaryTree {
	start: i32,
	end: i32,
	left: Option<Box<BinaryTree>>,
	right: Option<Box<BinaryTree>>,
}

#[allow(dead_code)]
impl BinaryTree {
	pub fn new(start: i32, end: i32) -> Self {
		Self {
			start,
			end,
			left: None,
			right: None,
		}
	}

	pub fn insert(&mut self, node: Box<BinaryTree>) -> bool {
		if self.end > node.start && node.end > self.start {
			false
		} else if node.start < self.start {
			if let Some(left) = &mut self.left {
				left.insert(node)
			} else {
				self.left = Some(node);
				true
			}
		} else if let Some(right) = &mut self.right {
			right.insert(node)
		} else {
			self.right = Some(node);
			true
		}
	}
}

#[allow(dead_code)]
struct MyCalendar {
	root: Option<Box<BinaryTree>>,
}

#[allow(dead_code)]
impl MyCalendar {
	fn new() -> Self {
		Self { root: None }
	}

	fn book(&mut self, start: i32, end: i32) -> bool {
		let curr = Box::new(BinaryTree {
			start,
			end,
			left: None,
			right: None,
		});

		if let Some(root) = &mut self.root {
			root.insert(curr)
		} else {
			self.root = Some(curr);
			true
		}
	}
}
