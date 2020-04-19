use std::error::Error;
use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
	let size = get_line()?.trim().parse::<usize>()?;
	let mut arr = vec![0usize; size];
	for (i, s) in get_line()?.split_whitespace().enumerate() {
		arr[s.parse::<usize>()? - 1] = i + 1;
	}
	print!("{}", arr[0]);
	for i in 1..arr.len() {
		print!(" {}", arr[i]);
	}
	Ok(())
}
