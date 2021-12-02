// Advent of Code 2021 Day 1 Solution by webbgamers#0246

pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
	(part1(&input), part2(&input))
}

fn part1(input: &String) -> Option<isize> {
	let depths = input.split_whitespace();
	let mut prev = 1000000000;
	let mut num = 0;
	for depth in depths {
		//println!("{:?}", depth.split("").collect::<String>());
		let depth = depth.parse::<i32>().unwrap();
		if depth > prev {
			num += 1;
		}
		prev = depth;
	}
	Option::Some(num)
}

fn part2(input: &String) -> Option<isize> {
	let depths = input.split_whitespace();
	let depths = depths.collect::<Vec<_>>();
	let mut i = 0;
	let mut prev = 1000000000;
	let mut num = 0;
	while i < depths.len() - 2 {
		let sum = depths[i].parse::<i32>().unwrap() + depths[i+1].parse::<i32>().unwrap() + depths[i+2].parse::<i32>().unwrap();

		if sum > prev { num += 1 }

		prev = sum;

		i += 1;
	}
	Option::Some(num)
}