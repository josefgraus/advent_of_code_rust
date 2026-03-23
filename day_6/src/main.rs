use common::download_input;
use std::fs;
use std::fmt;
use nalgebra::{DMatrix, DVector, DVectorView};

enum Alignment {
   Left,
   Right
}

// For debugging
impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Alignment::Left => write!(f, "Left <-"),
            Alignment::Right => write!(f, "Right ->")
        }
    }
}

fn parse_homework(worksheet: &[&str]) -> (Vec<char>, Vec<u128>) {
   // Find the line with operators and split it out from operands
   let (ops, values) = worksheet.split_last().unwrap();

   // Parse all operands into ints
   let values: Vec<u128> = values
      .into_iter()
      .flat_map(|s| s.split_whitespace()
         .map(|x| x.parse::<u128>().expect("item is invalid integer!")))
      .collect();   

   // Parse all operators into separate characters to be evaluated later
   let ops: Vec<char> = ops
      .split_whitespace()
      .flat_map(|s| s.chars())
      .collect();
      
   (ops, values)
}

// This function takes a column, determines the numeric value by reading vertically per digit, and converts those 
// recombinations back into numbers in a new column
fn vert(col: DVectorView<u128>, alignment: Alignment) -> DVector<u128> {
   let num_str: Vec<String> = match alignment {
      Alignment::Right => {
         // Reverse the numbers to deal with the alignment difference
         col.iter()
            .map(|x| x.to_string().chars().rev().collect())
            .collect()
      
      },
      Alignment::Left => {
         // Don't reverse the numbers since alignment is fine
         col.iter()
            .map(|x| x.to_string())
            .collect()
      }
   };

   let rows = num_str.len();  // Same number of rows as original
   let cols = num_str.iter().max_by_key(|s| s.len()).unwrap().len(); // We need to iterate over the largest string, and just pad for shorter ones

   // Here we're going to break each number string into its constituent characters and store them as u32 values in a matrix
   // We're going to then transpose the matrix and reconstitute the numbers by concatenating the rows and parsing the numbers
   let data: Vec<u32> = num_str.iter()
      .flat_map(|s| {
         let mut row: Vec<u32> = s.chars().map(|c| c as u32).collect();
         row.resize(cols, 0); // shadowing, briefly. Also a padding value of 0 is a control character we can filter out later
         row
      }).collect();

   // Turn our characters into a matrix, then transpose the values
   // Consider a 2x1 matrix of [ 12, 34 ], this operation would create a 2x2 matrix of,
   // | '1' '2' | (tranpose) => | '1' '3' |
   // | '3' '4' |               | '2' '4' |
   let char_mat = DMatrix::from_row_slice(rows, cols, &data);
   let transpose = char_mat.transpose();

   // This now takes the tranpose matrix, concatenates each row, and parses out a number
   // So using the previous comment example, the transposed matrix becomes [ 13, 24 ]
   let num: Vec<u128> = transpose.row_iter()
      .map(|row| {
         row.iter()
            .filter(|&&c| c != 0)
            .filter_map(|&c| char::from_u32(c))
            .collect::<String>()
            .parse()
            .unwrap()
      }).collect();

   DVector::from_row_slice(&num)
}

fn do_homework(worksheet: &[&str], vertical: bool) -> Vec<u128> {
   let (ops, values) = parse_homework(worksheet);
   let data_lines = &worksheet[..worksheet.len()-1];
   let op_line = worksheet.last().unwrap();

   // Construct a matrix from the operands
   let rows = worksheet.len() - 1;
   let cols = ops.len();
   let homework = DMatrix::from_row_slice(rows, cols, &values);

   // The puzzle is tricky because the numbers are left or right justified in their column assignment 
   // We account for this by checking if every row in the column with the operator contains a non-whitespace character
   // If each row is occupied, then the numbers are left aligned. Otherwise, they're right aligned.
   let op_char_positions: Vec<usize> = op_line
      .char_indices()
      .filter(|(_, c)| !c.is_whitespace())
      .map(|(i, _)| i)
      .collect();

   // Index into the operands given the column index to determine the operation to perform on the operands in each row
   // Return a vector of operation results
   homework.column_iter().enumerate().map(|(i, col)| {
      // Determine left/right alignment
      let op_pos = op_char_positions[i];

      // Check if every data row has a digit at the operator's character position
      let alignment = if data_lines.iter().all(|line| {
         line.chars().nth(op_pos)
               .map(|c| c.is_ascii_digit())
               .unwrap_or(false)
      }) { Alignment::Left } else { Alignment::Right };


      match ops[i] {
         '+' => if vertical { vert(col, alignment).sum() } else { col.sum() }
         '*' => if vertical { vert(col, alignment).product() } else { col.product() },
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

   let subtotal = do_homework(&worksheet, false);
   let total: u128 = subtotal.iter().map(|&x| x as u128).sum();
   println!("Sum of all operations horizontal is {total}");

   let subtotal = do_homework(&worksheet, true);
   let total: u128 = subtotal.iter().sum();
   println!("Sum of all operations vertical is {total}");
}

#[cfg(test)]
mod tests {
   use super::*;

   // Pulled from Advent of Code day 6 example
   // https://adventofcode.com/2025/day/6
   const INPUT: &[&str] = &[
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  "
   ];

   #[test]
   fn test_homework() {
      let mut given = vec![33210, 490, 4243455, 401];
      let given_total: u128 = 4277556;
      let mut subtotal = do_homework(&INPUT.to_vec(), false);
      let total: u128 = subtotal.iter().sum();

      given.sort();
      subtotal.sort();

      assert_eq!(given, subtotal);
      assert_eq!(given_total, total);
   }

   #[test]
   fn test_homework_vertical() {
      let mut given = vec![1058, 3253600, 625, 8544];
      let given_total: u128 = 3263827;
      let mut subtotal = do_homework(&INPUT.to_vec(), true);
      let total: u128 = subtotal.iter().sum();

      given.sort();
      subtotal.sort();

      assert_eq!(given, subtotal);
      assert_eq!(given_total, total);
   }
}