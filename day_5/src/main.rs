use common::download_input;
use std::fs;
use cs_utils::drain_filter;
use std::ops::RangeInclusive;

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
   if ranges.is_empty() {
      return vec![];
   }

   // Sort by start of the range
   ranges.sort_by_key(|r| *r.start());

   // We will start with the first range, sorted by start so it begins first numerically
   // We'll then check if each consecutive range overlaps it
   // If so, we'll merge the two ranges together, and continue
   // Else, we'll commit current to our merged vector, assign the next range as the new current
   // This continues until there are no more ranges to visit
   let mut merged = Vec::new();
   let mut current = ranges[0].clone();

   for r in &ranges[1..] {
      // Merge if overlapping or contiguous
      if *r.start() <= *current.end() + 1 { 
         // Extend current range if needed
         if *r.end() > *current.end() {
               current = *current.start()..=*r.end();
         }
      } else {
         // If r does not overlap with current, then push current, and set current to a mutable clone of r
         merged.push(current);
         current = r.clone();
      }
   }

   // Push final range
   merged.push(current);
   // Return our merged ranges
   merged
}

fn parse_inventory(inventory: &[&str]) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
   let (db, items): (Vec<&str>, Vec<&str>) = inventory
      .iter()
      .filter(|s| !s.trim().is_empty())
      .partition(|&s| s.contains('-'));

   let items: Vec<u64> = items
      .into_iter()
      .map(|s| s.parse::<u64>().expect("item is invalid integer!"))
      .collect();

   let mut ranges: Vec<RangeInclusive<u64>> = vec![];
   for range_str in db {
      let ends: Vec<&str> = range_str.split("-").collect();

      // A range can only have one start value and one end value
      assert!(ends.len() == 2);

      // Generate the range of number from its string representation
      let range = RangeInclusive::new(
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

      ranges.push(range);
   }

   (ranges, items)
}

fn check_ingredients(inventory: &[&str], exhaust: bool) -> usize {
   let (ranges, mut items) = parse_inventory(inventory);
   
   if !exhaust {
      let mut valid: Vec<u64> = vec![];
      for range in ranges {
         let mut drained = drain_filter(
            &mut items,
            |item| {
               range.contains(item)
            }
         );

         valid.append(&mut drained)
      }

      valid.len()
   } else {
      // Kept for posterity, little trap lain by the puzzle input
      // This is such a brute force wildly inefficient way to do this -- there is certainly a better way
      // Refactor this to merge all the ranges into disjoint intervals then just sum up the size of the interval (end - start) for all ranges
      // NOTE: This is done below as the true solution
      //let unique: HashSet<u64> = ranges.into_iter().flatten().collect();
      //valid = unique.into_iter().collect();
      //valid.len()

      let ranges = merge_ranges(ranges);
      ranges.iter().map(|r| (r.end() - r.start() + 1) as usize).sum()
   }
}

fn main() {
   let input = match fs::exists("day_5/input.txt") {
      Ok(_) => fs::read_to_string("day_5/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/5/input")
   };

   let ingredients: Vec<&str> = input.lines().collect();

   let fresh = check_ingredients(&ingredients, false);
   println!("Number of ingredients that are still fresh is {fresh}");

   let fresh = check_ingredients(&ingredients, true);
   println!("Number of ingredients that could possibly be still fresh is {fresh}");
}

#[cfg(test)]
mod tests {
   use super::*;

   // Pulled from Advent of Code day 5 example
   // https://adventofcode.com/2025/day/5
   const INPUT: &[&str] = &[
      "3-5",
      "10-14",
      "16-20",
      "12-18",
      "",
      "1",
      "5",
      "8",
      "11",
      "17",
      "32"
   ];

   #[test]
   fn test_single_category_fresh() {
      let given = vec![5, 11, 17].len();
      let fresh = check_ingredients(&INPUT.to_vec(), false);

      assert_eq!(given, fresh);
   }

   #[test]
   fn test_exhaustive_fresh() {
      let given = vec![3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20].len();
      let fresh = check_ingredients(&INPUT.to_vec(), true);

      assert_eq!(given, fresh);
   }
}