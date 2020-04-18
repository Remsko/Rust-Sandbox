use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let n = get_line()?.trim().parse::<usize>()?;
	println!("{}", n / 2);
	for _ in 1..n / 2 {
		print!("2 ");
	}
	println!("{}", if n % 2 == 0 { "2" } else { "3" });
	Ok(())
}
