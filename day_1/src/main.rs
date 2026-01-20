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
      // Parse number for rotation from line
      let rot = match line[1..].trim().parse::<i32>() {
         Ok(val) => val,
         Err(e) => {
            eprintln!("Could not convert {line}: {e}");
            continue;
         }
      };

      // Change dial based on rotation and direction
      if line.starts_with("L") {
         dial = (dial - rot + DIAL_CARDINALITY) % DIAL_CARDINALITY;
      } else if line.starts_with("R") {
         dial = (dial + rot) % DIAL_CARDINALITY;
      }

      // Did the last operation return the dial to value zero? Count it
      if dial == 0 {         
         zeroed += 1;
      }
   }

   println!("Number of times the dial was zero is {zeroed}");
}
