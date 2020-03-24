impl Solution {
	pub fn my_atoi(str: String) -> i32 {
		let mut s = str.chars().skip_while(|c| c.is_whitespace()).peekable();
		let mut sign = 1;

		match s.peek() {
			Some(&'+') => {
				s.next();
			}
			Some(&'-') => {
				sign = -1;
				s.next();
			}
			None => {
				return 0;
			}
			_ => {}
		};

		match s.peek() {
			Some(&c) => {
				if !c.is_ascii_digit() {
					return 0;
				}
			}
			None => {
				return 0;
			}
		};

		s.take_while(|c| c.is_ascii_digit())
			.collect::<String>()
			.parse::<i32>()
			.map(|n| n * sign)
			.unwrap_or(if sign == 1 {
				std::i32::MAX
			} else {
				std::i32::MIN
			})
	}
}