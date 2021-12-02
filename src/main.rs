use std::env;
use std::fs;

mod day01; mod day02; mod day03;

fn main() {
    let day = match env::args().nth(1) {
        Some(day) => day,
        None => { println!("You must specify a day."); return }
    };
    let day = match day.parse::<usize>() {
        Ok(day) => day,
        Err(_) => { println!("Enter a vaild day."); return }
    };
    
    let input = get_input(&day);
    
    let (p1, p2) = match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),

        d if d > 25 => { println!("Enter a day between 1 and 25."); return },
        _ => (None, None)
    };

    println!("\nDay {} Part 1:", day);
    println!("{}\n", match p1 { None => String::from("Not implemented yet"), Some(s) => s.to_string() });
    println!("Day {} Part 2:", day);
    println!("{}\n", match p2 { None => String::from("Not implemented yet"), Some(s) => s.to_string()} );
}

pub fn get_input(day: &usize) -> String {
    let fp = format!("input/day{:02}.txt", day);
	match fs::read_to_string(&fp) {
        Ok(input) => input,
        Err(error) => { println!("Failed to load input file '{}': {}", &fp, error); String::from("")}
    }
}
