use std::env;
use std::fs;

mod day01; mod day02;

fn main() {
    let day = env::args().nth(1).expect("What day?").parse::<usize>().expect("Enter a valid day");
    let input = get_input(&day);
    
    let (p1, p2) = match day {
        1 => day01::solve(input),
        2 => day02::solve(input),

        _ => panic!("Enter a valid day (0-25)")
    };

    println!("\nDay {} Part 1:", day);
    println!("{}\n", match p1 { None => String::from("Not implemented yet"), Some(s) => s.to_string() });
    println!("Day {} Part 2:", day);
    println!("{}\n", match p2 { None => String::from("Not implemented yet"), Some(s) => s.to_string()} );
}

pub fn get_input(day: &usize) -> String {
	fs::read_to_string(format!("input/day{:02}.txt", day)).expect("Failed to load input file")
}
