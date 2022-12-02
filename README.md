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

## December 01

Messed with structure types, fiddled with traits.

Blatant cut and paste of the file-reader code from stackoverflow.

Liked the operator overload for adding food to an Elf.

I got myself into a bit of a mess using `Vec` types so went with a statically sized array an part 2.

Still gettnig my head around pass-by-ref etc. (It's been many a year since I did any C++).

I like the idea of the Ok/Result types and used this as a side effect to detect the blank lines... a bit dirty.

Far from the most elegant of solutions, especially so in part 2 - I guess there are nice reducer type functionalities to discover.

Two correct answers though.

## December 02

I think I went `Option` crazy. `Some` and `None` everywhere - not sure I'm seing the value of this yet - kind of a way to 'cope' until you can't cope no more. The enum certainly overkill - but is sugary sweet syntax.
Did a parser for args in main.rs so you can now run any day using the command

```
cargo run -- <day>
```

Part 2 was much easier to adapt for today.
