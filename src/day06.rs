/**
 *--- Day 6: Tuning Trouble ---
The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb
After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
How many characters need to be processed before the first start-of-packet marker is detected?
 *
 */
use std::fs::File;
use std::path::Path;

use std::io::BufRead;
use std::io::BufReader;

pub fn run(part: u8) {
    println!("Welcome to day6, part {}... Tuning Trouble", part);
    match part {
        1 => solve(4),
        2 => solve(14),
        _ => panic!("Incorrect part"),
    }
}

fn uniq_test(slice: &[char]) -> usize {
    let len = slice.len();
    let mut skip = 0;
    for i1 in 0..len {
        for i2 in i1 + 1..len {
            if slice[len - 1 - i1] == slice[len - 1 - i2] {
                skip = len - i2
            }
            if skip > 0 {
                break;
            }
        }
        if skip > 0 {
            break;
        }
    }
    skip
}

fn solve(size: usize) {
    let lines: Vec<String> = lines_from_file("./input/day06.txt");
    let char_vec: Vec<char> = lines[0].chars().collect();
    //println!("{:?}", char_vec);
    assert!(char_vec.len() >= 4);
    let mut index = size - 1;
    let mut found = false;
    while !found {
        let skip = uniq_test(&char_vec[index + 1 - size..index + 1]);
        if skip == 0 {
            found = true
        } else {
            println!(
                "{:?} skipping {}",
                &char_vec[index + 1 - size..index + 1],
                skip
            );
            index += skip;
        }
    }
    println!(
        "Found {:?} ending at {}",
        &char_vec[index + 1 - size..index + 1],
        index + 1
    );
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
