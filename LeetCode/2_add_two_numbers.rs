impl Solution {
	pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut n1 = &l1;
		let mut n2 = &l2;
		
		let mut ret = None;
		let mut c = &mut ret;
		
		let mut carry = 0;
		while n1.is_some() || n2.is_some() || carry != 0 {
			let mut sum = carry;
			if let Some(l1) = n1 {
				sum += l1.val;
				n1 = &l1.next;
			}
			if let Some(l2) = n2 {
				sum += l2.val;
				n2 = &l2.next;
			}
			carry = sum / 10;
			*c = Some(Box::new(ListNode::new(sum % 10)));
			c = &mut c.as_mut().unwrap().next;
		}
		ret
	}
}