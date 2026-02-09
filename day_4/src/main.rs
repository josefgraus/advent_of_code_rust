use common::download_input;
use std::fs;
use std::collections::HashMap;
use nalgebra::DMatrix;

fn roll_neighborhood(roll_map: &DMatrix<u8>, i: usize, j: usize, k: usize, limit: usize) -> u64
{
   if roll_map[(i, j)] == 0 {
      return 0;
   }

   let i_0 = i.saturating_sub(k);
   let j_0 = j.saturating_sub(k);
   let dim = 2 * k + 1;
   let dim_i_0 = (i as i64 - k as i64).min(0).abs() as usize;
   let dim_j_0 = (j as i64 - k as i64).min(0).abs() as usize;

   let rows = (dim + i).min(roll_map.nrows().saturating_sub(i_0)).clamp(0, dim) - dim_i_0;
   let cols= (dim + j).min(roll_map.ncols().saturating_sub(j_0)).clamp(0, dim) - dim_j_0;

   let rolls: u64 = roll_map.view((i_0, j_0), (rows, cols)).iter().map(|&d| d as u64).sum();

   (rolls <= limit as u64) as u64
}

fn roll_access(rolls: &[&str], distance: usize, limit: usize) -> u64 {
   if rolls.len() <= 0 {
      return 0;
   }

   let symbols: HashMap<char, u8> = [
      ('.', 0),
      ('@', 1)
   ].into_iter().collect();

   let rows = rolls.len();
   let cols = rolls[0].len();
   
   let data: Vec<u8> = rolls
      .iter()
      .flat_map(|line| {
         line.chars().map(|c: char| {
            *symbols.get(&c).expect("Unknown symbol {c}")
         })
      }).collect();

   let roll_map = DMatrix::from_row_slice(rows, cols, &data);

   (0..roll_map.nrows())
      .flat_map(|i| (0..roll_map.ncols()).map(move |j: usize| (i, j)))
      .map(|(i, j)| 
         roll_neighborhood(&roll_map, i, j, distance, limit)
      ).sum()
}

fn main() {
   let input = match fs::exists("day_4/input.txt") {
      Ok(_) => fs::read_to_string("day_4/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/4/input")
   };

   let map: Vec<&str> = input.lines().collect();

   let rolls = roll_access(&map, 1, 4);
   println!("Accessible rolls in map {rolls}");
}

#[cfg(test)]
mod tests {
   use super::*;

   // Pulled from Advent of Code day 4 example
   // https://adventofcode.com/2025/day/3
   const INPUT: &[&str] = &[
      "..@@.@@@@.",
      "@@@.@.@.@@",
      "@@@@@.@.@@",
      "@.@@@@..@.",
      "@@.@@@@.@@",
      ".@@@@@@@.@",
      ".@.@.@.@@@",
      "@.@@@.@@@@",
      ".@@@@@@@@.",
      "@.@.@@@.@."
   ];

   #[test]
   fn test_max_rolls_near() {
      let given = 13;
      let rolls = roll_access(&INPUT.to_vec(), 1, 4);

      assert_eq!(given, rolls);
   }

}