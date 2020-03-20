impl Solution {
	pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
		let mut i = 0;
		let mut j = 0;
		let mut current = 0;
		let mut last = 0;
		let total = nums1.len() + nums2.len();
		let median =  total / 2;

		while i + j <= median {
			let num1 = nums1.get(i).or(Some(&std::i32::MAX)).unwrap();
			let num2 = nums2.get(j).or(Some(&std::i32::MAX)).unwrap();

			if num1 < num2 {
				i += 1;
				last = current;
				current = *num1;
			}
			else {
				j += 1;
				last = current;
				current = *num2;
			}
		}

		if total % 2 == 0 {
			(last + current) as f64 / 2.0
		}
		else {
			current as f64
		}
	}
}