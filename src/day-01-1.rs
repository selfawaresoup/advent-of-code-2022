use std::cmp::max;

static INPUT:&str = include_str!("day-01-input");


fn get_input() -> Vec<Vec<u64>> {
	let s = String::from(INPUT);
	let mut input = vec![];
	let lines = s.split("\n");

	let mut current = vec![];

	for line in lines {
		if let Ok(cal) = line.parse::<u64>() {
			current.push(cal);
		} else {
			input.push(current);
			current = vec![];
		}
	}

	input
}

fn main() {
	let mut max_cal = 0;

	for e in get_input() {
		let total: u64 = e.iter().sum();
		max_cal = max(total, max_cal);
	}
	
	println!("{max_cal}");
}
