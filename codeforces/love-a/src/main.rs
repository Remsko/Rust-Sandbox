use std::error::Error;
use std::io;

fn get_lines() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = get_lines()?;
	let s = input.trim();
	let a_count = s.chars().filter(|c| *c == 'a').count();
	let not_a_count = s.len() - a_count;
	print!(
		"{}",
		if a_count > not_a_count {
			s.len()
		} else {
			a_count * 2 - 1
		}
	);
	Ok(())
}
