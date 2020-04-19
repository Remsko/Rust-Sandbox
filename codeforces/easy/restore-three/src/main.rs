use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let input = get_line().unwrap();
	let mut x: Vec<_> = input
		.split_whitespace()
		.map(|s| s.parse::<u32>().unwrap())
		.collect();
	x.sort();
	let a = x[3] - x[0];
	let b = x[3] - x[1];
	let c = x[3] - x[2];
	print!("{} {} {}", a, b, c);
}
