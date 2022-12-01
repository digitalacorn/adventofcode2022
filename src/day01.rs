use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Elf {
    number: u32,
    carrying: u32,
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Elf #{} is carrying {}", self.number, self.carrying)
    }
}

impl ops::Add<u32> for Elf {
    type Output = Elf;

    fn add(self, rhs: u32) -> Elf {
        //println!("Adding {} food to Elf #{}", rhs, self.number);

        Elf {
            number: self.number,
            carrying: self.carrying + rhs,
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compare_elfs(current_best_elf: &Elf, contender_elf: &Elf) -> Elf {
    if contender_elf.carrying > current_best_elf.carrying || current_best_elf.carrying == 0 {
        contender_elf.clone()
    } else {
        current_best_elf.clone()
    }
}

fn update_worst_best_elf(top_elves: &mut [Elf], contender_elf: &Elf) {
    let mut worst_carrying: u32 = 99999999;
    let mut worst_idx: usize = 0;
    for index in 0..top_elves.len() {
        if top_elves[index].carrying <= worst_carrying {
            worst_idx = index;
            worst_carrying = top_elves[index].carrying;
        }
    }

    top_elves[worst_idx] = compare_elfs(&top_elves[worst_idx], contender_elf);
}

pub fn run() {
    println!("Welcome to day1!");
    let mut best_elf = Elf {
        number: 0,
        carrying: 0,
    };
    let mut top_elves: [Elf; 3] = [best_elf; 3];
    let mut current_elf = Elf {
        number: 1,
        carrying: 0,
    };
    if let Ok(lines) = read_lines("./input/day01.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(food_string) = line {
                if let Ok(food) = food_string.parse::<u32>() {
                    //println!("{}", food);
                    current_elf = current_elf + food;
                } else {
                    // Blank line cannot be converted to an u32
                    best_elf = compare_elfs(&best_elf, &current_elf);
                    update_worst_best_elf(&mut top_elves, &current_elf);

                    println!("{}", current_elf);
                    current_elf = Elf {
                        number: current_elf.number + 1,
                        carrying: 0,
                    };
                }
            } else {
                println!("Line read fail");
            }
        }
        println!("{}", current_elf);
        best_elf = compare_elfs(&best_elf, &current_elf);
        update_worst_best_elf(&mut top_elves, &current_elf);

        let mut top_elves_carry: u32 = 0;
        for index in 0..top_elves.len() {
            top_elves_carry += top_elves[index].carrying;
            println!("One of our top elves is: {}", top_elves[index]);
        }
        println!("Betweeen them they carry: {}", top_elves_carry);
    } else {
        println!("Failed to read file");
    }
    println!("Our best Elf is: {}", best_elf);
}
