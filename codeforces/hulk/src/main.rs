use std::io;

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let input = get_input().unwrap();
	let mut split = input.split_whitespace();

	let mut i = split.next().expect("").parse::<u32>().unwrap();
	let mut grognon = true;

	while i > 0 {
		i -= 1;
		if grognon {
			print!("I hate {}", if i == 0 { "it" } else { "that " });
		} else {
			print!("I love {}", if i == 0 { "it" } else { "that " });
		}
		grognon = !grognon;
	}
}
