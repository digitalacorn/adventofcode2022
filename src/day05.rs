/**
 * --- Day 5: Supply Stacks ---
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
 *
 */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(part: u8) {
    println!("Welcome to day5, part {}... Supply Stacks", part);
    match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("Incorrect part"),
    }
}

fn render_stacks(stacks: &[Vec<char>; 10]) {
    for idx in 0..9 {
        println!("{} = {:?}", idx + 1, stacks[idx]);
    }
}

fn part1() {
    let mut lines = read_lines("./input/day05.txt").unwrap();
    let mut header_section = true;
    let mut stacks: [Vec<char>; 10] = Default::default();

    // cop out - assume 9 stacks
    for idx in 0..9 {
        stacks[idx] = Vec::new();
    }

    while header_section {
        if let Ok(input) = lines.next().unwrap() {
            if input.len() > 0 {
                // cop out - assert input is correct width
                assert_eq!(input.len(), 35);
                // cop out - assume utf-8
                let char_vec: Vec<char> = input.chars().collect();
                for idx in 0..9 {
                    if char_vec[idx * 4] == '[' {
                        stacks[idx].push(char_vec[idx * 4 + 1]);
                        stacks[idx].rotate_right(1);
                    }
                }

                println!("header {}", input)
            } else {
                header_section = false;
            }
        }
    }
    render_stacks(&stacks);
    for line in lines {
        if let Ok(input) = line {
            if input.len() > 0 {
                let clean: String = input
                    .chars()
                    .filter(|c| c.is_digit(10) || c.is_whitespace())
                    .collect::<String>();
                let clean2 = clean.split_whitespace();
                let parts = clean2.map(|v| v.parse::<u32>().unwrap());
                let array: [u32; 3] = parts.collect::<Vec<u32>>().try_into().unwrap();
                let (num, from, to) = (array[0], array[1], array[2]);
                println!("moving x{:?} from {:?} to {:?}", num, from, to);
                for _ in 0..num {
                    let item = stacks[(from - 1) as usize].pop();
                    match item {
                        None => panic!("stack {} went empty", from),
                        _ => stacks[(to - 1) as usize].push(item.unwrap()),
                    }
                }
                render_stacks(&stacks);
            } else {
            }
        }
    }
    render_stacks(&stacks);

    print!("Stack Tops: ");
    for idx in 0..9 {
        print!("{}", stacks[idx].pop().unwrap());
    }
    println!("");
}

fn part2() {
    let mut lines = read_lines("./input/day05.txt").unwrap();
    let mut header_section = true;
    let mut stacks: [Vec<char>; 10] = Default::default();

    // cop out - assume 9 stacks
    for idx in 0..9 {
        stacks[idx] = Vec::new();
    }

    while header_section {
        if let Ok(input) = lines.next().unwrap() {
            if input.len() > 0 {
                // cop out - assert input is correct width
                assert_eq!(input.len(), 35);
                // cop out - assume utf-8
                let char_vec: Vec<char> = input.chars().collect();
                for idx in 0..9 {
                    if char_vec[idx * 4] == '[' {
                        stacks[idx].push(char_vec[idx * 4 + 1]);
                        stacks[idx].rotate_right(1);
                    }
                }

                println!("header {}", input)
            } else {
                header_section = false;
            }
        }
    }
    render_stacks(&stacks);

    let mut tmp_lazy_stack: Vec<char> = Vec::new();
    for line in lines {
        if let Ok(input) = line {
            if input.len() > 0 {
                let clean: String = input
                    .chars()
                    .filter(|c| c.is_digit(10) || c.is_whitespace())
                    .collect::<String>();
                let clean2 = clean.split_whitespace();
                let parts = clean2.map(|v| v.parse::<u32>().unwrap());
                let array: [u32; 3] = parts.collect::<Vec<u32>>().try_into().unwrap();
                let (num, from, to) = (array[0], array[1], array[2]);
                println!("moving x{:?} from {:?} to {:?}", num, from, to);
                for _ in 0..num {
                    let item = stacks[(from - 1) as usize].pop();
                    match item {
                        None => panic!("stack {} went empty", from),
                        _ => tmp_lazy_stack.push(item.unwrap()),
                    }
                }
                for _ in 0..num {
                    let item = tmp_lazy_stack.pop();
                    match item {
                        None => panic!("stack {} went empty", from),
                        _ => stacks[(to - 1) as usize].push(item.unwrap()),
                    }
                }
                render_stacks(&stacks);
            } else {
            }
        }
    }
    render_stacks(&stacks);

    print!("Stack Tops: ");
    for idx in 0..9 {
        print!("{}", stacks[idx].pop().unwrap());
    }
    println!("");
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
