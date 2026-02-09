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

fn max_joltages(banks: &[&str], magnitude: usize) -> Vec<u64> {
   let mut values: Vec<u64> = Vec::new();
   for &s in banks {
      assert!(s.len() >= magnitude);

      let mut index: usize = 0;
      let mut rindex = s.len() - magnitude;
      let mut value: u64 = 0;
      while index < s.len() && rindex < s.len() {
         let (f_index, largest) = first_index_of_largest(&s[index..=rindex]);
         index += f_index+1;
         rindex += 1;
         value += (largest as u64) * (10 as u64).pow((s.len() - rindex) as u32);
      }
      values.push(value);
   }
   values
}

fn main() {
   let input = match fs::exists("day_3/input.txt") {
      Ok(_) => fs::read_to_string("day_3/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/3/input")
   };

   let banks: Vec<&str> = input.lines().collect();

   let joltages_pair = max_joltages(&banks, 2);
   let joltages_pair_sum: u64 = joltages_pair.iter().map(|&x| x).sum();

   println!("Max possible output pair joltage is {joltages_pair_sum}");

   let joltages_dodeca = max_joltages(&banks, 12);
   let joltages_dodeca_sum: u64 = joltages_dodeca.iter().map(|&x| x).sum();

   println!("Max possible output dodeca joltage is {joltages_dodeca_sum}");
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
   fn test_max_joltage_pair() {
      let given: Vec<u64> = vec![98, 89, 78, 92];
      let joltages = max_joltages(&INPUT.to_vec(), 2);

      assert_eq!(given, joltages);
      assert_eq!(given.iter().sum::<u64>(), joltages.iter().sum::<u64>());
   }

}