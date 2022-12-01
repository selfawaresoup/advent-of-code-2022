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
	let mut elves = get_input();

	elves.sort_by(|b, a| a.iter().sum::<u64>().cmp(&b.iter().sum::<u64>()));

	elves.truncate(3);

	let mut max_cal = 0;

	for e in elves {
		let total: u64 = e.iter().sum();
		max_cal += total;
	}
	
	println!("{max_cal}");
}
