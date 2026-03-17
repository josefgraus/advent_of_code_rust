use common::download_input;
use std::fs;
use nalgebra::DMatrix;

fn parse_homework(worksheet: &[&str]) -> (Vec<char>, Vec<u64>) {
   // Find the line with operators and split it out from operands
   let (ops, values): (Vec<&str>, Vec<&str>) = worksheet
      .iter()
      .partition(|&s| s.contains('*') || s.contains("+"));

   // There should only be one row of operations
   assert!(ops.len() == 1);

   // Parse all operands into ints
   let values: Vec<u64> = values
      .into_iter()
      .flat_map(|s| s.split_whitespace()
         .map(|x| x.parse::<u64>().expect("item is invalid integer!")))
      .collect();   

   // Parse all operators into separate characters to be evaluated later
   let ops: Vec<char> = ops[0]
      .split_whitespace()
      .flat_map(|s| s.chars())
      .collect();
      
   (ops, values)
}

fn do_homework(worksheet: &[&str]) -> Vec<u64> {
   let (ops, values) = parse_homework(worksheet);

   // Construct a matrix from the operands
   let rows = worksheet.len() - 1;
   let cols = ops.len();
   let homework = DMatrix::from_row_slice(rows, cols, &values);

   // Index into the operands given the column index to determine the operation to perform on the operands in each row
   // Return a vector of operation results
   homework.column_iter().enumerate().map(|(i, col)| {
      match ops[i] {
         '+' => col.sum(),
         '*' => col.product(),
         _ => panic!("Unrecognized numeric operation {}!", ops[i])
      }
   }).collect()
}

fn main() {
   let input = match fs::exists("day_6/input.txt") {
      Ok(_) => fs::read_to_string("day_6/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/5/input")
   };

   let worksheet: Vec<&str> = input.lines().collect();

   let subtotal = do_homework(&worksheet);
   let total: u64 = subtotal.iter().sum();
   println!("Sum of all operations is {total}");
}

#[cfg(test)]
mod tests {
   use super::*;

   // Pulled from Advent of Code day 6 example
   // https://adventofcode.com/2025/day/6
   const INPUT: &[&str] = &[
        "123 328  51 64",
        "45 64  387 23",
        "6 98  215 314",
        "*   +   *   +"
   ];

   #[test]
   fn test_single_category_fresh() {
      let given = vec![33210, 490, 4243455, 401];
      let given_total: u64 = given.iter().sum();
      let subtotal = do_homework(&INPUT.to_vec());
      let total: u64 = subtotal.iter().sum();

      assert_eq!(given, subtotal);
      assert_eq!(given_total, total);
   }

   #[test]
   fn test_exhaustive_fresh() {
   }
}