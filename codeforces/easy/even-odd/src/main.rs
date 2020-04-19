use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let input = get_line().unwrap();
	let n: u32 = input.trim().parse().unwrap();

	print!("{}", if n % 2 == 0 { "Mahmoud" } else { "Ehab" });
}
