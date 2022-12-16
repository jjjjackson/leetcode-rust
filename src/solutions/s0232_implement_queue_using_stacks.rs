struct Solution {}

struct MyQueue {
	values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
	fn new() -> Self {
		Self { values: Vec::new() }
	}

	fn push(&mut self, x: i32) {
		self.values.push(x)
	}

	fn pop(&mut self) -> i32 {
		self.values.remove(0)
	}

	fn peek(&self) -> i32 {
		*self.values.first().unwrap()
	}

	fn empty(&self) -> bool {
		self.values.is_empty()
	}
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
	use test_case::test_case;

	fn cases() {}
}
