use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let employees;
	{
		let input = get_line().unwrap();
		employees = input.trim().parse::<u32>().unwrap();
	}

	let mut ways = 0;
	for i in 1..employees {
		if (employees - i) % i == 0 {
			ways += 1;
		}
	}
	print!("{}", ways);
}
