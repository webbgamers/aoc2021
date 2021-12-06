// Advent of Code 2021 Day 6 Solution by webbgamers#0246

use std::collections::HashMap;

pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
	(part1(&input), part2(&input))
}

fn part1(input: &String) -> Option<isize> {
    let mut fish = input.split(',').map(|f| f.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut i = 0;
    while i <= 80 {
        let mut newfish: Vec<usize> = Vec::new();
        for f in &fish {
            match f {
                0 => { newfish.push(8); newfish.push(6); },
                _ => newfish.push(f - 1)
            }
        }
        fish = newfish;
        i += 1;
    }

    Some(fish.len() as isize)
}

fn part2(input: &String) -> Option<isize> {
	let fish = input.split_whitespace().collect::<String>();
    let fish = fish.split(',').map(|f| f.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut fishmap: HashMap<usize, usize> = HashMap::new();

    for f in fish {
        *fishmap.entry(f).or_insert(0) += 1;
    }

    let mut i = 0;
    while i < 256 {
        let mut newfish: HashMap<usize, usize> = HashMap::new();
        for f in fishmap {
            match f.0 {
                0 => { *newfish.entry(8).or_insert(0) += f.1; *newfish.entry(6).or_insert(0) += f.1; },
                _ => { *newfish.entry(f.0 - 1).or_insert(0) += f.1; }
            }
        }
        fishmap = newfish;
        i += 1;
        /*let mut fishcount = 0;
        for f in &fishmap {
            fishcount += f.1;
        }
        println!("Iteration {} complete.", i);
        println!("Fish count: {}", fishcount)*/
    }

    let mut fishcount = 0;
    for f in fishmap {
        fishcount += f.1;
    }

    Some(fishcount as isize)
}