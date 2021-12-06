// Advent of Code 2021 Day 5 Solution by webbgamers#0246

use std::collections::HashMap;

pub fn solve(input: String) -> (Option<isize>, Option<isize>) {
	(part1(&input), part2(&input))
}

fn part1(input: &String) -> Option<isize> {
    // Extremely readable input parser
	let input = input.lines().map(|l| l.split(" -> ").map(str::to_owned).map(|s| s.split(',').map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>()).map(|l| Line{x1: l[0][0].parse().unwrap(), y1: l[0][1].parse().unwrap(), x2: l[1][0].parse().unwrap(), y2: l[1][1].parse().unwrap()});
    // Filter out diagonals
    let input = input.filter(|l| l.x1 == l.x2 || l.y1 == l.y2);

    let mut ventmap: HashMap<(usize, usize), usize> = HashMap::new();

    for l in input {
        let mut x = l.x1;
        let mut y = l.y1;
        *ventmap.entry((x, y)).or_insert(0) += 1;
        while x != l.x2 || y != l.y2 {
        
            match x as isize - l.x2 as isize {
                d if d < 0 => x += 1,
                d if d > 0 => x -= 1,
                _ => {},
            }

            match y as isize - l.y2 as isize {
                d if d < 0 => y += 1,
                d if d > 0 => y -= 1,
                _ => {},
            }

            *ventmap.entry((x, y)).or_insert(0) += 1;
        }
    }

    let mut dc = 0;

    for pos in ventmap {
        if pos.1 > 1 {
            dc += 1;
        }
    }

    Some(dc)
}

fn part2(input: &String) -> Option<isize> {
	// Extremely readable input parser
	let input = input.lines().map(|l| l.split(" -> ").map(str::to_owned).map(|s| s.split(',').map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>()).map(|l| Line{x1: l[0][0].parse().unwrap(), y1: l[0][1].parse().unwrap(), x2: l[1][0].parse().unwrap(), y2: l[1][1].parse().unwrap()});
    // Filter out diagonals
    //let input = input.filter(|l| l.x1 == l.x2 || l.y1 == l.y2);

    let mut ventmap: HashMap<(usize, usize), usize> = HashMap::new();

    for l in input {
        let mut x = l.x1;
        let mut y = l.y1;
        *ventmap.entry((x, y)).or_insert(0) += 1;
        while x != l.x2 || y != l.y2 {
        
            match x as isize - l.x2 as isize {
                d if d < 0 => x += 1,
                d if d > 0 => x -= 1,
                _ => {},
            }

            match y as isize - l.y2 as isize {
                d if d < 0 => y += 1,
                d if d > 0 => y -= 1,
                _ => {},
            }

            *ventmap.entry((x, y)).or_insert(0) += 1;
        }
    }

    let mut dc = 0;

    for pos in ventmap {
        if pos.1 > 1 {
            dc += 1;
        }
    }

    Some(dc)
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}