use std::io;

fn get_input(input: &mut Vec<String>) -> io::Result<()> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	let mut split: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();
	input.append(&mut split);
	Ok(())
}

fn main() -> io::Result<()> {
	let mut input: Vec<String> = Vec::new();
	get_input(&mut input)?;
	let mut n = input[0].parse::<i32>().unwrap();
	let mut k = input[1].parse::<i32>().unwrap();
	while k > 0 {
		if n % 10 == 0 {
			n /= 10;
		} else {
			n -= 1;
		}
		k -= 1;
	}
	print!("{}", n);
	Ok(())
}
