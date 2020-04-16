use std::error::Error;
use std::io;

fn get_lines() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

#[inline(always)]
fn manhattan_distance(xa: i8, xb: i8, ya: i8, yb: i8) -> i8 {
	i8::abs(xb - xa) + i8::abs(yb - ya)
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut matrix = vec![];
	let mut x = 0;
	let mut y = 0;
	for i in 0..5 {
		let rows: Vec<u8> = get_lines()?
			.split_whitespace()
			.map(|s| s.parse::<u8>().unwrap())
			.collect();
		for j in 0..5 {
			if rows[j] == 1 {
				x = j;
				y = i;
			}
		}
		matrix.push(rows);
	}
	print!("{}", manhattan_distance(x as i8, 2, y as i8, 2));
	Ok(())
}
