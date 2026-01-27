use common::download_input;
use std::fs;
//use std::hint::black_box;

// Simple convenience function for converting num_str into a u64 and checking if it's equal to num
fn is_equal(num: u64, num_str: &str) -> bool {
   let x = match num_str.trim().parse::<u64>() {
      Ok(val) => val,
      Err(e) => {
         eprintln!("Could not convert {}: {e}", num_str);
         return false;
      }
   };

   x == num
}

fn invalid_ids(input: &str, all_repeats: bool) -> Vec<u64> {
   // Separate each input range by a comma delimter
   let input_ranges = input.split(",");

   let mut invalid: Vec<u64> = Vec::new();
   for input_range in input_ranges {
      // Separate each range into start and end values
      let ends: Vec<&str> = input_range.split("-").collect();

      // A range can only have one start value and one end value
      assert!(ends.len() == 2);

      // Generate the range of number from its string representation
      let range = std::ops::RangeInclusive::new(
         match ends[0].trim().parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
               eprintln!("Could not convert {}: {e}", ends[0]);
               continue;
            }
         },
         match ends[1].trim().parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
               eprintln!("Could not convert {}: {e}", ends[1]);
               continue;
            }
         }
      );   
      
      // Iterate over the range and determine what values in that range qualify as "invalid" from the puzzle instructions
      // Both part 1 and part 2 (toggled by the "all_repeats" bool -- false for part 1, true for part 2) have similar logic 
      // to testing for palindromes way back in intro comp sci days. We simply subdivide the string, repeat the first subdivision N-times,
      // convert the value back to a u64, then see if it is equal to the original number. If it is, it's "invalid" and is added to the returned vector.
      for num in range {
         let num_str = num.to_string();

         if all_repeats {
            // Part 2
            // Progressively takes a bigger and bigger substring and repeats it up to the length of the original value then checks if the values are equal
            for i in 1..(num_str.len()/2)+1 {
               let sub = String::from(&num_str[0..i]);
               let repeat = sub.repeat(num_str.len() / sub.len());
               
               // Normally I delete out debug stuff that has served its function, but I thought this black_box() function was a cool discovery
               // It strongly hints that the rust compiler should not, if at all possible, optimize out a variable you'd like to inspect
               // I was having trouble inspecting variables even using the "`dev` profile [unoptimized + debuginfo]", but this allowed me to do so.
               // https://doc.rust-lang.org/std/hint/fn.black_box.html
               //black_box(&repeat);
               
               if is_equal(num, &repeat) && !invalid.contains(&num) {
                  invalid.push(num);
               }
            }
         } else {
            // Part 1
            // Simply split the string in half then see if repeating it twice equals the original value
            if num_str.len() % 2 != 0 {
               continue;
            }

            let mid = num_str.len() / 2;
            let half: String = String::from(&num_str[..mid]);
            let repeat = half.repeat(2);

            if is_equal(num, &repeat) {
               invalid.push(num);
            }
         }
      }
   }

   invalid
}

fn main() {
   let input = match fs::exists("day_2/input.txt") {
      Ok(_) => fs::read_to_string("day_2/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/2/input")
   };

   let ids_pairs = invalid_ids(&input, false);
   let ids_pairs_sum: u64 = ids_pairs.iter().sum();

   let ids_nths = invalid_ids(&input, true);
   let ids_nths_sum: u64 = ids_nths.iter().sum();

   println!("Sum of the invalid IDs made up of the same number twice is {ids_pairs_sum}");
   println!("Sum of the invalid IDs made up of a number repeated N times is {ids_nths_sum}");
}

#[cfg(test)]
mod tests {
   use super::*;
   use indoc::indoc;

   // Pulled from Advent of Code day 2 example
   // https://adventofcode.com/2025/day/2
   const INPUT: &str = indoc!{"
      11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
   "};

   #[test]
   fn test_invalid_ids_pairs() {
      // Ground truth for INPUT from Advent of Code day 2, part 1 example
      let invalid = vec!(
         11, 22,
         99,
         1010,
         1188511885,
         222222,
         446446,
         38593859
      );
      let invalid_sum: u64 = invalid.iter().sum();

      let ids = invalid_ids(INPUT, false);
      let ids_sum: u64 = ids.iter().sum();

      assert_eq!(invalid.len(), ids.len());

      for id in ids {
         assert_eq!(invalid.contains(&id), true);
      }

      assert_eq!(invalid_sum, ids_sum);
   }

   #[test]
   fn test_invalid_ids_nths() {
      // Ground truth for INPUT from Advent of Code day 2, part 1 example
      let invalid = vec!(
         11, 22,
         99, 111,
         999, 1010,
         1188511885,
         222222,
         446446,
         38593859,
         565656,
         824824824,
         2121212121
      );
      let invalid_sum: u64 = invalid.iter().sum();

      let ids = invalid_ids(INPUT, true);
      let ids_sum: u64 = ids.iter().sum();

      assert_eq!(invalid.len(), ids.len());

      for id in ids {
         assert_eq!(invalid.contains(&id), true);
      }

      assert_eq!(invalid_sum, ids_sum);
   }
}