use std::cmp::Ordering;
use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let _ = get_line()?;
	let games = get_line()?;
	let mut danik = 0;
	let mut anton = 0;
	for c in games.chars() {
		match c {
			'D' => {
				danik += 1;
			}
			'A' => {
				anton += 1;
			}
			_ => {}
		}
	}
	match anton.cmp(&danik) {
		Ordering::Greater => println!("Anton"),
		Ordering::Less => println!("Danik"),
		Ordering::Equal => println!("Friendship"),
	}
	Ok(())
}
