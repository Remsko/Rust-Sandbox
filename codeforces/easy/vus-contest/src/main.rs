use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let input = get_line().unwrap();
	let n: Vec<i16> = input
		.split_whitespace()
		.map(|s| s.parse::<i16>().unwrap())
		.collect();

	print!(
		"{}",
		if n[1] >= n[0] && n[2] >= n[0] {
			"Yes"
		} else {
			"No"
		}
	);
}
