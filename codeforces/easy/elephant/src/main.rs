use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let mut input = get_line().unwrap().trim().parse::<u32>().unwrap();
	let steps = vec![5, 4, 3, 2, 1];

	let mut total = 0;
	for i in 0..steps.len() {
		let d = steps[i];
		total += input / d;
		input %= d;
	}
	println!("{}", total);
}
