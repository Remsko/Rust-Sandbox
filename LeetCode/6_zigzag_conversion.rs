impl Solution {
	pub fn convert(s: String, num_rows: i32) -> String {
		if num_rows == 1 {
			return s;
		}
		let cycle_len = num_rows * 2 - 2;
		let sv: Vec<char> = s.chars().collect();
		let mut result = String::new();

		for row in 0..num_rows {
			let mut cycle = 0;

			while cycle + row < sv.len() as i32 {
				let middle = (cycle + cycle_len - row) as usize;
				
				result.push(sv[(cycle + row) as usize]);
				if row != 0 && row != num_rows - 1 && middle < sv.len() {
					result.push(sv[middle]);
				}
				cycle += cycle_len;
			}
		}
		result
	}
}