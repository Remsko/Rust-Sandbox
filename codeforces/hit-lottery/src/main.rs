use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let mut input = get_line().unwrap().trim().parse::<u32>().unwrap();
	let bills = vec![100, 20, 10, 5, 1];

	let mut total = 0;
	for i in 0..bills.len() {
		let d = bills[i];
		total += input / d;
		input %= d;
	}
	println!("{}", total);
}
