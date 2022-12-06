# Advent of Rust

I decided to look at Rust and that Advent of Code would be a good way to learn some basics.

# Diary

## November 30

Dowloaded rust compiler and got Hello World Running

Create a new project with

```
cargo new projectname
```

and compile and run using

```
cargo run
```

VS code plugin https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Day 01 - Calorie Counting

[source](./src/day01.rs)

Messed with structure types, fiddled with traits.

Blatant cut and paste of the file-reader code from stackoverflow.

Liked the operator overload for adding food to an Elf.

I got myself into a bit of a mess using `Vec` types so went with a statically sized array an part 2.

Still gettnig my head around pass-by-ref etc. (It's been many a year since I did any C++).

I like the idea of the Ok/Result types and used this as a side effect to detect the blank lines... a bit dirty.

Far from the most elegant of solutions, especially so in part 2 - I guess there are nice reducer type functionalities to discover.

Two correct answers though.

## Day 02 - Rock Paper Scissors

[source](./src/day02.rs)

I think I went `Option` crazy. `Some` and `None` everywhere - not sure I'm seing the value of this yet - kind of a way to 'cope' until you can't cope no more. The enum certainly overkill - but is sugary sweet syntax.
Did a parser for args in main.rs so you can now run any day using the command

```
cargo run -- <day>
```

Part 2 was much easier to adapt for today.

## Day 03 - Rucksack Reorganization

[source](./src/day03.rs)

Updated main to add a closure to extract command line args and pass the part to the 'day' module's run function. Going forward each puzzle solution will support running both parts. (I overrwrote code for part 2 on days 1 and 2)

```
cargo run -- <day> <part>
```

Added assertions. Stopped getting hung up on Options. Went for what seems like an opimal solution - iterating over both compartments simultaneously ( `O(n)` ) and using a map for checking, rather than hunting for each item from one compartment in the other compartment ( `O(n^2)` ) (my first approach)

Made the initial mistake of forgetting that 2+ identical items in one side is perfectly valid.

Got my bitwise storage game on for part 2. Managed to overflow u8 like a noob until I went with a double cast (obvs with hindsight)

```rust
let elf_num: u8 = (index as u32 % 3) as u8;
```

## Day 04 - Camp Cleanup

[source](./src/day04.rs)

Easy one once I rembered to parse string pointers into u32 integers.

## Day 5 - Supply Stacks

[source](./src/day05.rs)

Fighting with vectors unwrapping, mapping, types. :rollseyes:
I'm a day behind so went for a lazy solution using an interim Vec 'tower of hanoi' style solution to do the moves. Instead of learning some slice methods.
