/**
 * --- Day 3: Rucksack Reorganization ---
One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
The fourth rucksack's compartments only share item type v.
The fifth rucksack's compartments only share item type t.
The sixth rucksack's compartments only share item type s.
To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?


 */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LOWER_A: u32 = 'a' as u32;
const LOWER_Z: u32 = 'z' as u32;
const UPPER_A: u32 = 'A' as u32;

pub fn run(part: u8) {
    println!("Welcome to day3, part {}... Rucksacks", part);
    match part {
        1 => part1(),
        2 => part2(),
        _ => panic!("Incorrect part"),
    }
}

fn priority0based(item: char) -> u8 {
    let i: u32 = item as u32;
    if i >= LOWER_A && i <= LOWER_Z {
        (i - LOWER_A).try_into().unwrap()
    } else {
        (26 + i - UPPER_A).try_into().unwrap()
    }
}

fn part2() {
    let mut total_priority: u32 = 0;

    // bitwise storage for which elves have this item
    let mut trio_map: [u8; 52] = [0; 52];

    if let Ok(lines) = read_lines("./input/day03.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(rucksack) = line {
                let elf_num: u8 = (index as u32 % 3) as u8;
                //println!("{} {} {}", index, elf_num, rucksack);
                let mut found_item: (char, u8) = (' ', 0);
                for item in rucksack.chars() {
                    let p1 = priority0based(item);
                    trio_map[p1 as usize] |= 1 << elf_num;
                    if trio_map[p1 as usize] == 7 {
                        found_item = (item, p1 + 1);
                        break;
                    }
                }
                if elf_num == 2 {
                    let (item, priority) = found_item;
                    println!(
                        "Found item {} with priority {} in each of the 3 rows preceeding row {}",
                        item,
                        priority,
                        index + 1
                    );
                    total_priority += priority as u32;
                    trio_map = [0; 52];
                }
            }
        }
    }
    println!("Total priority of badges {}", total_priority);
}

fn part1() {
    let mut total_priority: u32 = 0;

    if let Ok(lines) = read_lines("./input/day03.txt") {
        for line in lines {
            if let Ok(rucksack) = line {
                let compartment_len = rucksack.len() / 2;
                let (compartment1, compartment2) =
                    (&rucksack[..compartment_len], &rucksack[compartment_len..]);
                assert_eq!(compartment1.len(), compartment2.len());

                // maps which compartment each item (indexed by priorty) is in
                let mut priority_map: [u8; 52] = [0; 52];
                let mut found: bool = false;
                let mut found_item: (char, u8) = (' ', 0);
                let mut c1 = compartment1.chars();
                let mut c2 = compartment2.chars();
                while !found {
                    let i1 = c1.next().unwrap();
                    let p1 = priority0based(i1);

                    if priority_map[p1 as usize] == 2 {
                        // this compartment one item already in compartment two
                        found = true;
                        found_item = (i1, p1 + 1);
                    } else {
                        priority_map[p1 as usize] = 1;
                        let i2 = c2.next().unwrap();

                        let p2 = priority0based(i2);
                        if priority_map[p2 as usize] == 1 {
                            // this compartment two item already in compartment one
                            found = true;
                            found_item = (i2, p2 + 1);
                        } else {
                            priority_map[p2 as usize] = 2;
                        }
                    }
                }
                let (item, priority) = found_item;
                println!(
                    "Found item {} with priority {} in both {} and {}",
                    item, priority, compartment1, compartment2
                );
                total_priority += priority as u32;
            }
        }
    }
    println!("Total priority {}", total_priority);
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
