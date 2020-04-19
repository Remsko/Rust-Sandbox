use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();

	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let _ = get_line().unwrap();
	let input = get_line().unwrap();

	print!(
		"{}",
		if input.split_whitespace().filter(|c| *c == "1").count() > 0 {
			"HARD"
		} else {
			"EASY"
		},
	);
}
