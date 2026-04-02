use std::fs;
use common::download_input;

fn rect_area(p: &[i64;2], q: &[i64;2]) -> i64 {
   ((p[0] - q[0]).abs() +1) * ((p[1] - q[1]).abs() + 1)
}

fn largest_rect(points: &[&str]) -> [[i64;2]; 2] {
   // Convert each line containing a 3D coordinate into an [i64; 3]
   let points: Vec<[i64; 2]> = points.iter()
      .map(|s| {
         s.split(",")
            .map(|x| x.parse::<i64>().expect("coordinate components are not valid integers!"))
            .collect::<Vec<_>>()
            .try_into().expect("Each line of input must contain two comma delimited integers!")
      })
      .collect();

   assert!(points.len() >= 2);

   // Brute force O(n^2), return pair of points that create the largest rectangle
   let mut best_pair: [[i64; 2]; 2] = points[..2].try_into().unwrap();
   let mut best_area = rect_area(&best_pair[0], &best_pair[1]);
   for p1 in points.iter() {
      for p2 in points.iter() {
         let area = rect_area(p1, p2);
         (best_pair, best_area) = if best_area >= area { (best_pair, best_area) } else { ([p1.clone(), p2.clone()], area) };
      }
   }

   best_pair
}

fn main() {
   let input = match fs::exists("day_9/input.txt") {
      Ok(_) => fs::read_to_string("day_9/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/9/input")
   };

   let points: Vec<&str> = input.lines().collect();
   let [p, q] = largest_rect(&points);
   let best_area = rect_area(&p,&q);
   println!("The largest area rectangle that can be created it {best_area}");
}

#[cfg(test)]
mod tests {
   use super::*;
   use indoc::indoc;

   // Pulled from Advent of Code day 9 example
   // https://adventofcode.com/2025/day/9
   const INPUT: &str = indoc!{"
      7,1
      11,1
      11,7
      9,7
      9,5
      2,5
      2,3
      7,3
   "};

   #[test]
   fn test_() {
      let input: Vec<&str> = INPUT.lines().collect();
      let mut given_pair = [[2,5], [11,1]];
      let given_area = 50;

      let mut pair = largest_rect(&input);
      let calc_area = rect_area(&pair[0], &pair[1]);

      pair.sort();
      given_pair.sort();

      let test_area = rect_area(&[0,0], &[5,5]);
      assert_eq!(test_area, 36);

      let test_area = rect_area(&[1,0], &[5,5]);
      assert_eq!(test_area, 30);

      assert_eq!(pair, given_pair);
      assert_eq!(calc_area, given_area);
   }
}