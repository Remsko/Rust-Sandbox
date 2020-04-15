use std::io;

fn get_input() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn poly_size(poly: &str) -> usize {
	match poly {
		"Tetrahedron" => 4,
		"Cube" => 6,
		"Octahedron" => 8,
		"Dodecahedron" => 12,
		"Icosahedron" => 20,
		_ => 0,
	}
}

fn main() {
	let mut i = get_input().unwrap().trim().parse::<u32>().unwrap();
	let mut total = 0;
	while i > 0 {
		let input = get_input().unwrap();
		total += poly_size(input.trim());
		i -= 1;
	}
	print!("{}", total);
}
