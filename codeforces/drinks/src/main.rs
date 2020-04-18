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
	let percentage: f64 = input.iter().sum::<u32>() as f64 / input.len() as f64;
	print!("{}", percentage);
	Ok(())
}
