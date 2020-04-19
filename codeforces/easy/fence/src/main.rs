use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let hence = get_line()?.split_whitespace().collect::<Vec<_>>()[1].parse::<u16>()?;
	let width = get_line()?.split_whitespace().fold(0, |acc, s| {
		let height = s.parse::<u16>().unwrap();
		acc + if height > hence { 2 } else { 1 }
	});
	print!("{:?}", width);
	Ok(())
}
