use common::download_input;
use std::fs;

fn first_index_of_largest(nbr_string: &str) -> (usize, u32) {
   nbr_string.chars()
      .enumerate()
      .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
      .fold(None, |max, (i, d)| match max {
            None => Some((i, d)),
            Some((_, max_d)) if d > max_d => Some((i, d)),
            Some((max_i, max_d)) => Some((max_i, max_d)), // keep the first max
      })
      .expect("Failed to find first index of largest digit in string!")
}

fn max_joltages(bank: &Vec<&str>) -> Vec<u32> {
   bank.iter()
      .map(|s| {
         let (index, forward_max) = first_index_of_largest(&s[..s.len()-1]);

         let reversed: String = s[index+1..].chars().rev().collect();
         
         let (_rev_index, reverse_max) = first_index_of_largest(&reversed);

         forward_max * 10 + reverse_max
      }).collect()
}

fn main() {
   let input = match fs::exists("day_3/input.txt") {
      Ok(_) => fs::read_to_string("day_3/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/3/input")
   };

   let joltages = max_joltages(&input.lines().collect());
   let joltages_sum = joltages.iter().sum::<u32>();

   println!("Max possible output joltage is {joltages_sum}")
}

#[cfg(test)]
mod tests {
   use super::*;

   // Pulled from Advent of Code day 2 example
   // https://adventofcode.com/2025/day/3
   const INPUT: &[&str] = &[
      "987654321111111",
      "811111111111119",
      "234234234234278",
      "818181911112111"
   ];

   #[test]
   fn test_() {
      let given: Vec<u32> = vec![98, 89, 78, 92];
      let joltages = max_joltages(&INPUT.to_vec());

      assert_eq!(given, joltages);
      assert_eq!(given.iter().sum::<u32>(), joltages.iter().sum::<u32>());
   }

}