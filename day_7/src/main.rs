use std::fs;
use common::download_input;
use std::collections::HashSet;

fn beam_split(diagram: &[&str]) -> u64 {
   let (start, field) = diagram.split_first().unwrap();

   let mut beams: HashSet<usize> = start
      .char_indices()
      .filter(|(_, c)| *c == 'S')
      .map(|(i, _)| i)
      .collect();

   let mut num_split = 0;

   for row in field {
      beams = beams.iter()
         .map(|i| {
            let mut travel: HashSet<usize> = HashSet::new();
            let space = row.chars().nth(*i);
            match space {
               Some(s) => {
                  match s {
                     '^' => {
                        if *i > 0 { travel.insert(*i - 1); }
                        if (*i + 1) < row.len() { travel.insert(*i + 1); }
                        num_split += 1;
                        travel
                     },
                     _ => {
                        travel.insert(*i);
                        travel
                     }
                  }
               },
               None => { panic!("We are out of bounds? There are no characters??"); }
            }
         })
         .flatten()
         .collect();
   }

   num_split
}

fn main() {
   let input = match fs::exists("day_7/input.txt") {
      Ok(_) => fs::read_to_string("day_7/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/7/input")
   };

   let diagram: Vec<&str> = input.lines().collect();

   let num_splits = beam_split(&diagram);
   println!("The beam split {num_splits} times.");
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
   fn test_homework() {
      let input: Vec<&str> = INPUT.lines().collect();
      let num_splits = beam_split(&input);

      assert_eq!(num_splits, 21);
   }
}