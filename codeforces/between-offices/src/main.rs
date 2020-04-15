use std::error::Error;
use std::io;

fn get_lines() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let _ = get_lines()?;
	let input = get_lines()?;
	let mut iter = input.chars();

	let mut s_to_sf = 0;
	let mut sf_to_s = 0;
	let mut last = iter.next().ok_or("NO")?;
	for c in iter {
		match (last, c) {
			('S', 'F') => {
				s_to_sf += 1;
			}
			('F', 'S') => {
				sf_to_s += 1;
			}
			_ => {}
		}
		last = c;
	}
	print!("{}", if s_to_sf > sf_to_s { "YES" } else { "NO" });
	Ok(())
}
