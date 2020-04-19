use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = get_line()?.trim().parse::<u32>()?;
	let mut groups = 1;

	let mut last = get_line()?;
	lines -= 1;
	while lines > 0 {
		let magnet = get_line()?;
		if magnet != last {
			last = magnet;
			groups += 1;
		}
		lines -= 1;
	}
	print!("{}", groups);
	Ok(())
}
