use common::download_input;
use std::fs;

const DIAL_CARDINALITY: i32 = 100;

fn dial_zeroes(input: &str, on_stop_only: bool) -> u64 {
   // Puzzle states we start with the dial pointing at the value 50
   let mut dial: i32 = 50;
   let mut zeroed: u64 = 0;
   let mut zero_pass: u64 = 0;
   for line in input.lines() {
      if line.is_empty() {
         continue;
      }

      // Parse number for rotation from line
      let rot = match line[1..].trim().parse::<i32>() {
         Ok(val) => val,
         Err(e) => {
            eprintln!("Could not convert {line}: {e}");
            continue;
         }
      };

      // Determine the rotation direction and apply the rotation based on the prefix
      let dial_next = match line.chars().next().unwrap() {
         'R' | 'r' => dial + rot,
         'L' | 'l' => dial - rot,
         _ => {
            // Prefix not recognized. Ignore this command, and continue the loop as though it doesn't exist
            continue;
         }
      };

      // If we haven't already counted landing on zero
      if dial != 0 && dial_next != 0 {
         // Check if we crossed over local zero (XOR of integer signs)
         if (dial.signum() > -1 && dial_next.signum() == -1) || (dial.signum() == -1 && dial_next.signum() > -1) {
            zero_pass += 1;
         }
      }

      // Leave dial in valid range [-99, 99] for next iteration
      dial = dial_next % DIAL_CARDINALITY;

      // Check how many times we can divide the dial by DIAL_CARDINALITY (discarding remainder)
      // as an indication of how many complete rotations it achieved from the spin operation.
      let inc_val = (dial_next / DIAL_CARDINALITY).abs() as u64;
      zero_pass += inc_val;

      // Did the last operation return the dial to value zero? Count it
      if dial == 0 {
         zeroed += 1;
         
         // Be careful not to double-count large rotation values that land on zero
         if inc_val > 0 {
            zero_pass -= 1;
         }
      }
   }

   // Return requested value
   match on_stop_only {
      true => zeroed,
      false => zeroed + zero_pass
   }
}

fn main() {
   let input = match fs::exists("day_1/input.txt") {
      Ok(_) => fs::read_to_string("day_1/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/1/input")
   };

   println!("Number of times the dial stopped on zero is {}", dial_zeroes(&input, true));
   println!("Number of times the dial pointed at zero at any time is {}", dial_zeroes(&input, false));
}

#[cfg(test)]
mod tests {
   // Note this useful idiom: importing names from outer (for mod tests) scope.
   use super::*;
   use indoc::indoc;

   // Pulled from Advent of Code day 1 example
   // https://adventofcode.com/2025/day/1
   const INPUT: &str = indoc!{"
      L68 
      L30 
      R48 
      L5 
      R60 
      L55 
      L1 
      L99 
      R14 
      L82 
   "};

   #[test]
   fn test_day1_example() {
      // Ground truth "3" of Advent of Code day 1, part 1 example
      assert_eq!(dial_zeroes(INPUT, true), 3); 
   }

   #[test]
   fn test_day2_example() {
      // Ground truth "6" of Advent of Code day 1, part 2 example
      assert_eq!(dial_zeroes(INPUT, false), 6);
   }
}