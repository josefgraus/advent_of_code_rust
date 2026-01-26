use common::download_input;
use std::fs;

fn invalid_ids(input: &str) -> Vec<u64> {
   let input_ranges = input.split(",");

   let mut invalid: Vec<u64> = Vec::new();
   for input_range in input_ranges {
      let ends: Vec<&str> = input_range.split("-").collect();

      assert!(ends.len() == 2);

      let range = std::ops::RangeInclusive::new(
         match ends[0].parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
               eprintln!("Could not convert {}: {e}", ends[0]);
               continue;
            }
         },
         match ends[1].parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
               eprintln!("Could not convert {}: {e}", ends[1]);
               continue;
            }
         }
      );   
      
      for num in range {
         let num_str = num.to_string();

         if num_str.len() % 2 != 0 {
            continue;
         }

         let mid = num_str.len() / 2;
         let half = String::from(&num_str[..mid]);
         let repeat = format!("{}{}", half, half);

         let x = match repeat.parse::<u64>() {
            Ok(val) => val,
            Err(e) => {
               eprintln!("Could not convert {}: {e}", ends[0]);
               continue;
            }
         };

         if x == num {
            invalid.push(num);
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

   let ids = invalid_ids(&input);
   let ids_sum: u64 = ids.iter().sum();

   println!("Sum of the invalid IDs is {ids_sum}.");
}

#[cfg(test)]
mod tests {
   use super::*;
   use indoc::indoc;

   const INPUT: &str = indoc!{"
      11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
   "};

   #[test]
   fn test_invalid_ids() {
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

      let ids = invalid_ids(INPUT);
      let ids_sum: u64 = ids.iter().sum();

      assert_eq!(invalid.len(), ids.len());

      for id in ids {
         assert_eq!(invalid.contains(&id), true);
      }

      assert_eq!(invalid_sum, ids_sum);
   }
}