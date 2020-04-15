use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let _ = get_line().unwrap();
	let input = get_line().unwrap();

	let (mut z, mut n) = (0, 0);
	for c in input.chars() {
		match c {
			'z' => {
				z += 1;
			}
			'n' => {
				n += 1;
			}
			_ => {}
		}
	}

	let mut print = false;
	while n > 0 {
		print!("{}{}", if print { " " } else { "" }, "1");
		print = true;
		n -= 1;
	}
	while z > 0 {
		print!("{}{}", if print { " " } else { "" }, "0");
		print = true;
		z -= 1;
	}
}
