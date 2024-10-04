/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
	data: Vec<T>,
}

impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
	}
	fn pop(&mut self) -> Option<T> {
		self.data.pop()
	}
	fn peek(&self) -> Option<&T> {
		self.data.last()
	}
}

fn bracket_match(brackets: &str) -> bool {
	let mut stack = Stack::new();
	for ch in brackets.chars() {
		match ch {
			'(' | '{' | '[' => stack.push(ch),
			')' => if stack.pop() != Some('(') { return false; },
			'}' => if stack.pop() != Some('{') { return false; },
			']' => if stack.pop() != Some('[') { return false; },
			_ => continue, // 忽略其他字符
		}
	}
	stack.is_empty() // 结束时检查栈是否为空，确保所有括号都已匹配
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s), true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s), true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s), true);
	}
}
