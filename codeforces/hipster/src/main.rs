use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = get_line()?
		.split_whitespace()
		.map(|s| s.parse::<u32>())
		.collect::<Result<Vec<_>, _>>()?;
	let diff = input[0].min(input[1]);
	let same = (input[0].max(input[1]) - diff) / 2;
	print!("{} {}", diff, same);
	Ok(())
}
