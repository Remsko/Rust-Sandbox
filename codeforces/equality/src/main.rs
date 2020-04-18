use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let _ = get_line()?;
	let input = get_line()?
		.split_whitespace()
		.map(|s| s.parse::<u32>())
		.collect::<Result<Vec<_>, _>>()?;
	let max = input.iter().max().ok_or("ERROR")?;
	let distance = input.iter().fold(0, |s, &x| s + max - x);
	print!("{}", distance);
	Ok(())
}
