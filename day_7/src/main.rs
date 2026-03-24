use std::fs;
use common::download_input;
use std::collections::HashSet;

// This function pulls double duty, returning two values in a tuple, (num_split, num_timelines)
// num_splits is the number of times that the beam was split when encountering a splitter ('^')
// num_timelines is the sum of all possible paths a single particle could take before arriving at the bottom
fn beam_split(diagram: &[&str]) -> (u64, u64) {
   // Pull off top row and assume there is at least one 'S' character to indicate start of beam
   let (start, field) = diagram.split_first().unwrap();

   // Setup a hashset that represents the indices of the row containing a beam
   // This starts with just the indices of the 'S' elements
   let mut beams: HashSet<usize> = start
      .char_indices()
      .filter(|(_, c)| *c == 'S')
      .map(|(i, _)| i)
      .collect();

   // Set num_splits to zero, and initialize timelines to be a row of zeros with a 1 in the position of each 'S' character
   // These 1's represent a single timeline from the particle start location
   let mut num_split = 0;
   let mut timelines: Vec<u64> = (0..start.len())
      .map(|i| if beams.contains(&i) { 1 } else { 0 })
      .collect();

   // For each row, find all splitters ('^') and update our beams and timelines appropriately
   for row in field {
      // We iterate over our current beams, and build a new beams set that gets assigned over the old one
      beams = beams.iter()
         .map(|i| {  // For each beam index
            let mut travel: HashSet<usize> = HashSet::new(); // Create a new hashset just for this index
            let space = row.chars().nth(*i);   // Get the character at this index in the row
            match space {
               Some(s) => {
                  match s {
                     '^' => {
                        // If a splitter ('^'), insert two new beams into the travel hashset and 
                        // add the timeline path at this index to the left and right indices (to denote the path splitting), 
                        // then zero out the timeline count at this index to denote the path terminating
                        if *i > 0 { 
                           travel.insert(*i - 1);
                           timelines[*i-1] += timelines[*i];
                        }
                        if (*i + 1) < row.len() { 
                           travel.insert(*i + 1);
                           timelines[*i+1] += timelines[*i];
                        }
                        timelines[*i] = 0;
                        num_split += 1;
                        travel
                     },
                     _ => {
                        // Otherwise there are no obstacles, so propagate the beam directly down
                        travel.insert(*i);
                        travel
                     }
                  }
               },
               None => { panic!("We are out of bounds? There are no characters??"); }
            }
         })
         .flatten() 
         .collect(); // Flatten and collect to eliminate duplicate indices so we don't overcount
   }

   // The answer is just num_split, and the sum of the values left in the timelines vector
   (num_split, timelines.iter().sum())
}

fn main() {
   let input = match fs::exists("day_7/input.txt") {
      Ok(_) => fs::read_to_string("day_7/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/7/input")
   };

   let diagram: Vec<&str> = input.lines().collect();

   let (num_splits, num_timelines) = beam_split(&diagram);
   println!("The beam split {num_splits} times.");
   println!("A single particle will travel {num_timelines} timelines.")
}

#[cfg(test)]
mod tests {
   use super::*;
   use indoc::indoc;

   // Pulled from Advent of Code day 7 example
   // https://adventofcode.com/2025/day/7
   const INPUT: &str = indoc!{"
      .......S.......
      ...............
      .......^.......
      ...............
      ......^.^......
      ...............
      .....^.^.^.....
      ...............
      ....^.^...^....
      ...............
      ...^.^...^.^...
      ...............
      ..^...^.....^..
      ...............
      .^.^.^.^.^...^.
      ...............
   "};

   #[test]
   fn test_splits() {
      let input: Vec<&str> = INPUT.lines().collect();
      let (num_splits, _) = beam_split(&input);

      assert_eq!(num_splits, 21);
   }

   #[test]
   fn test_timelines() {
      let input: Vec<&str> = INPUT.lines().collect();
      let (_, num_timelines) = beam_split(&input);

      assert_eq!(num_timelines, 40);
   }
}