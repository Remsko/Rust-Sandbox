use std::io;

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let input = get_input().unwrap();
	let mut split = input.split_whitespace();

	let mut a = split.next().expect("").parse::<u32>().unwrap();
	let mut b = split.next().expect("").parse::<u32>().unwrap();
	let mut i = 0;
	while a <= b {
		a *= 3;
		b *= 2;
		i += 1;
	}
	print!("{}", i);
}
