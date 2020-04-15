use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let mut lines: u8 = get_line().unwrap().trim().parse().unwrap();

	while lines > 0 {
		let input: u32 = get_line().unwrap().trim().parse().unwrap();
		let n;
		if input == 2 {
			n = 2;
		} else if input % 2 == 0 {
			n = 0;
		} else {
			n = 1;
		}
		print!("{}\n", n);
		lines -= 1;
	}
}
