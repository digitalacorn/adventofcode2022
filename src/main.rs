use std::env;

mod day01;
mod day02;
mod day03;
mod day04;

const DEFAULT_DAY: u8 = 4;
const DEFAULT_PART: u8 = 1;

fn get_day_part() -> (u8, u8) {
    let args: Vec<String> = env::args().collect();

    let get_param = |index: usize, default: u8| match args.get(index) {
        Some(str) => {
            if let Ok(val) = str.parse::<u8>() {
                val
            } else {
                panic!("Invalid parameter")
            }
        }
        None => default,
    };

    let day: u8 = get_param(1, DEFAULT_DAY);
    let part: u8 = get_param(2, DEFAULT_PART);

    if day < 3 && part != 2 {
        panic!("Only part 2 suppoerted before day 3.")
    }

    (day, part)
}

fn main() {
    println!("Advent of Rust");

    let (day, part) = get_day_part();

    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(part),
        4 => day04::run(part),
        u8::MAX | _ => panic!("Invalid day"),
    }
}
