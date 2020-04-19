use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let n = get_line()?.trim().parse::<usize>()?;
	let mut table = vec![1; n];
	for _ in 1..n {
		for q in 1..n {
			table[q] += table[q - 1];
		}
	}
	print!("{}", table[n - 1]);
	Ok(())
}
