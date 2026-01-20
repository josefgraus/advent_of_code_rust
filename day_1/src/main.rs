use common::download_input;
use std::fs;

const DIAL_CARDINALITY: i32 = 100;

fn main() {
   let input = match fs::exists("day_1/input.txt") {
      Ok(_) => fs::read_to_string("day_1/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/1/input")
   };

   // Puzzle states we start with the dial pointing at the value 50
   let mut dial: i32 = 50;
   let mut zeroed: u32 = 0;
   for line in input.lines() {
      // (1) This gets a little more dense than I want a statement to be since I'm not only parsing the number from a string slice
      //     but I'm also taking the absolute value of its remainder against the number of dial digits.
      //     Arguably a confusing read, not really succinct -- I haven't figured out the Rust preference for these kinds of statements
      let rot = match line[1..].trim().parse::<i32>() {
         Ok(val) => val,
         Err(e) => {
            eprintln!("Could not convert {line}: {e}");
            continue;
         }
      };

      if line.starts_with("L") {
         dial = (dial - rot + DIAL_CARDINALITY) % DIAL_CARDINALITY;
      } else if line.starts_with("R") {
         dial = (dial + rot) % DIAL_CARDINALITY;
      }

      if dial == 0 {         
         zeroed += 1;
      }
   }

   println!("Number of times the dial was zero is {zeroed}");
}
