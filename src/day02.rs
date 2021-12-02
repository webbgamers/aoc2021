// Advent of Code 2021 Day 2 Solution by webbgamers#0246

pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
	(part1(&input), part2(&input))
}

fn part1(input: &String) -> Option<isize> {
	let input = input.lines();
	let mut x = 0;
	let mut d = 0;
	for l in input {
		let mut l = l.split_whitespace();
		let command = l.next().unwrap();
		let val = l.next().unwrap().parse::<usize>().unwrap();

		match command {
			"forward" => x += val,
			"down" => d += val,
			"up" => d -= val,
			_ => panic!("What")
		}
	}
	Option::Some((x * d) as isize)
}

fn part2(input: &String) -> Option<isize> {
	let input = input.lines();
	let mut x = 0;
	let mut d = 0;
	let mut aim = 0;
	for l in input {
		let mut l = l.split_whitespace();
		let command = l.next().unwrap();
		let val = l.next().unwrap().parse::<usize>().unwrap();

		match command {
			"forward" => { x += val; d += aim*val },
			"down" => aim += val,
			"up" => aim -= val,
			_ => panic!("What")
		}
	}
	Option::Some((x * d) as isize)
}