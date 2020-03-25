impl Solution {
	pub fn is_palindrome(mut x: i32) -> bool {
		if x == 0 {
			return true;
		}
		if x < 0 || x % 10 == 0 {
			return false;
		}

		let mut reverse = 0;
		while x > reverse {
			reverse = reverse * 10 + x % 10;
			x /= 10;
		}
		x == reverse || x == reverse / 10 
	}
}