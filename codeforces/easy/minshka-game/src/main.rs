use std::cmp::Ordering;
use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut turns = get_line()?.trim().parse::<u8>()?;
	let mut mishka = 0;
	let mut chris = 0;
	while turns > 0 {
		let input = get_line()?
			.split_whitespace()
			.map(|s| s.parse::<u8>())
			.collect::<Result<Vec<_>, _>>()?;
		match input[0].cmp(&input[1]) {
			Ordering::Greater => mishka += 1,
			Ordering::Less => chris += 1,
			Ordering::Equal => {}
		}
		turns -= 1;
	}
	print!(
		"{}",
		match mishka.cmp(&chris) {
			Ordering::Greater => {
				"Mishka"
			}
			Ordering::Less => {
				"Chris"
			}
			Ordering::Equal => {
				"Friendship is magic!^^"
			}
		}
	);
	Ok(())
}
