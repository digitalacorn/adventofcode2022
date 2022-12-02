/**
 * --- Day 1: Calorie Counting ---
Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
The second Elf is carrying one food item with 4000 Calories.
The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
The fifth Elf is carrying one food item with 10000 Calories.
In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?


--- Part Two ---
By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

 */
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
    let mut worst_carrying: u32 = u32::MAX;
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
