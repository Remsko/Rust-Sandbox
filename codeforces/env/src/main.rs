use std::error::Error;
use std::io;
use std::str::FromStr;

fn get_line() -> io::Result<String> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;
	Ok(buffer)
}

fn get_input_number<T>() -> Result<T, Box<dyn Error + 'static>>
where
	T: FromStr,
	<T as FromStr>::Err: Error + 'static,
{
	Ok(get_line()?.trim().parse::<T>()?)
}

fn get_input_array<T>() -> Result<Vec<T>, Box<dyn Error + 'static>>
where
	T: FromStr,
	<T as FromStr>::Err: Error + 'static,
{
	Ok(get_line()?
		.split_whitespace()
		.map(|s| s.parse::<T>())
		.collect::<Result<Vec<_>, _>>()?)
}

fn fill_conj(conj: &mut Vec<Vec<usize>>, n: usize) -> Result<(), Box<dyn Error>> {
	for _ in 1..n {
		let input2 = get_input_array::<usize>()?;
		let u = input2[0];
		let v = input2[1];
		conj[u].push(v);
		conj[v].push(u);
	}
	Ok(())
}

fn dfs(
	u: usize,
	f: usize,
	conj: &mut Vec<Vec<usize>>,
	depth: &mut Vec<usize>,
	size: &mut Vec<usize>,
	det: &mut Vec<i64>,
) -> usize {
	depth[u] = depth[f] + 1;
	size[u] = 1;
	for i in 0..conj[u].len() {
		let v = conj[u][i];
		if v == f {
			continue;
		}
		size[u] += dfs(v, u, conj, depth, size, det);
	}
	det[u] = size[u] as i64 - depth[u] as i64;
	size[u]
}

fn main() -> Result<(), Box<dyn Error>> {
	let input1 = get_input_array::<usize>()?;
	let n = input1[0];
	let k = input1[1];
	let mut conj = vec![vec![0usize; 0]; n + 1];
	fill_conj(&mut conj, n)?;
	let mut depth = vec![0usize; n + 1];
	let mut size = vec![0usize; n + 1];
	let mut det = vec![0i64; n + 1];
	dfs(1, 0, &mut conj, &mut depth, &mut size, &mut det);
	det[1..].sort_by(|a, b| b.cmp(a));
	let mut ans = 0;
	for i in 1..n - k {
		ans += det[i]
	}
	println!("{}", ans);
	Ok(())
}
