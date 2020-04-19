use std::error::Error;
use std::io;

fn get_lines() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn split_lines(lines: String) -> Vec<String> {
	return lines.split_whitespace().map(|s| s.to_string()).collect();
}

fn main() -> Result<(), Box<dyn Error>> {
	let _ = get_lines()?;
	let input1: Vec<String> = split_lines(get_lines()?);
	let input2: Vec<String> = split_lines(get_lines()?);
	let mut lines = get_lines().unwrap().trim().parse::<u16>()?;

	while lines > 0 {
		let query = get_lines()?.trim().parse::<usize>()?;
		let left = &input1[(query - 1) % input1.len()];
		let right = &input2[(query - 1) % input2.len()];
		println!("{}{}", left, right);
		lines -= 1;
	}
	Ok(())
}
