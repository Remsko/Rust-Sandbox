use std::io;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn main() {
	let input = get_line().unwrap();
	let card = input.trim().as_bytes();
	let input = get_line().unwrap();
	let hand: Vec<&[u8]> = input.split_whitespace().map(|s| s.as_bytes()).collect();
	let mut play = "NO";
	for i in 0..hand.len() {
		if card[0] == hand[i][0] || card[1] == hand[i][1] {
			play = "YES";
			break;
		}
	}
	print!("{}", play);
}
