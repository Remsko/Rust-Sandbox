use std::error::Error;
use std::io;

fn get_lines() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut lines = get_lines()?.trim().parse::<u8>()?;
	while lines > 0 {
		let input = get_lines()?;
		let sentence = input.trim();
		if sentence.ends_with("po") {
			println!("FILIPINO");
		} else if sentence.ends_with("desu") || sentence.ends_with("masu") {
			println!("JAPANESE");
		} else if sentence.ends_with("mnida") {
			println!("KOREAN");
		}
		lines -= 1;
	}
	Ok(())
}
