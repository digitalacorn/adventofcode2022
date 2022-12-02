use std::env;

mod day01;
mod day02;

const DEFAULT_DAY: u8 = 2;

fn get_day() -> u8 {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(day_str) => {
            if let Ok(day) = day_str.parse::<u8>() {
                day
            } else {
                u8::MAX
            }
        }
        None => DEFAULT_DAY,
    }
}

fn main() {
    println!("Advent of Rust");

    match get_day() {
        1 => day01::run(),
        2 => day02::run(),
        u8::MAX | _ => println!("Invalid day."),
    }
}
