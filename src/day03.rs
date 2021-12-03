// Advent of Code 2021 Day 3 Solution by webbgamers#0246

pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
	(part1(&input), part2(&input))
}

fn part1(input: &String) -> Option<isize> {
	let input = input.lines();
	let mut bitarray: Vec<Vec<char>> = Vec::new();

	for l in input {
		bitarray.push(l.chars().collect());
	}

	let mut cbits: Vec<char> = Vec::new();

	let mut i = 0;
	while i < bitarray[0].len() {
		let mut count = 0;

		let mut j = 0;
		while j < bitarray.len() {
			if bitarray[j][i] == '0' { count += 1 }
			j += 1;
		}
		cbits.push( if count > bitarray.len() / 2 { '0' } else { '1' });
		i += 1;
	}

	let num: String = cbits.iter().collect();
	let gr = usize::from_str_radix(&num, 2).unwrap();
	let er = { 
		let mut cbits: Vec<char> = Vec::new();
		for c in num.chars() {
			cbits.push( match c {
				'0' => '1',
				'1' => '0',
				_ => panic!("What even")
			});
		}
		let num: String = cbits.iter().collect();
		usize::from_str_radix(&num, 2).unwrap()
	};
	println!("{:?}", cbits);
	Some((gr*er) as isize)
}

fn part2(input: &String) -> Option<isize> {
	let input = input.lines();
	let mut bitarray: Vec<Vec<char>> = Vec::new();

	for l in input {
		bitarray.push(l.chars().collect());
	}

	// filter for OGR
	let mut ogr_array = bitarray.to_vec();
	let mut i = 0;
	while ogr_array.len() > 1 {
		let mut new_array: Vec<Vec<char>> = Vec::new();
		let mut bits: Vec<char> = Vec::new();
		for line in &ogr_array {
			bits.push(line[i])
		}
		let cbit = get_cbit(&bits);

		for line in &ogr_array {
			if &line[i] == &cbit {
				new_array.push(line.to_vec())
			}
		}
		ogr_array = new_array.to_vec();
		if ogr_array.len() < 20 {
			println!("iteration {}", i);
			println!("{:?}", ogr_array)
		}
		i += 1;
	}

	// filter for CSR
	let mut csr_array = bitarray.to_vec();
	let mut i = 0;
	while csr_array.len() > 1 {
		let mut new_array: Vec<Vec<char>> = Vec::new();
		let mut bits: Vec<char> = Vec::new();
		for line in &csr_array {
			bits.push(line[i])
		}
		let cbit = match get_cbit(&bits) { '0' => '1', '1' => '0', _ => panic!("What even")};

		for line in &csr_array {
			if &line[i] == &cbit {
				new_array.push(line.to_vec())
			}
		}
		csr_array = new_array.to_vec();
		if csr_array.len() < 20 {
			println!("iteration {}", i);
			println!("{:?}", csr_array)
		}
		i += 1;
	}

	let ogr: String = ogr_array[0].iter().collect();
	let ogr = usize::from_str_radix(&ogr, 2).unwrap();

	let csr: String = csr_array[0].iter().collect();
	let csr = usize::from_str_radix(&csr, 2).unwrap();

	Some((ogr*csr) as isize)
}

fn get_cbit(bitarray: &Vec<char>) -> char {
	let mut count = 0;
	for bit in bitarray {
		if bit == &'0' {
			count += 1;
		}
	}
	if count > bitarray.len() / 2 { '0' } else { '1' }
}