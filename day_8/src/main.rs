use std::fs;
use common::download_input;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug)]
struct Edge {
   a: [i64; 3],
   b: [i64; 3]
}

impl Edge {
   fn new(a: [i64; 3], b: [i64; 3]) -> Self {
      Edge { a, b }
   }

   fn _norm_sq(&self) -> i64 {
      self.a.iter()
         .zip(self.b.iter())
         .map(|(x, y)| x * y)
         .sum()
   }
}

impl PartialEq for Edge {
   fn eq(&self, other: &Self) -> bool {
      (self.a == other.a && self.b == other.b) ||
      (self.a == other.b && self.b == other.a)
   }
}

impl Hash for Edge {
   fn hash<H: Hasher>(&self, state: &mut H) {
      // Hash both orderings the same way by sorting the two endpoints
      let (first, second) = if self.a <= self.b {
         (self.a, self.b)
      } else {
         (self.b, self.a)
      };
      first.hash(state);
      second.hash(state);
   }
}

fn largest_components(junction_boxes: &[&str], num_edges: usize, num_components: usize) -> Vec<i64> {
   let items: Vec<[i64; 3]> = junction_boxes.iter()
      .map(|s| {
         s.split(",")
            .map(|x| x.parse::<i64>().expect("coordinate components are not valid integers!"))
            .collect::<Vec<_>>()
            .try_into().expect("Each line of input must contain three whitespace delimited integers!")
      })
      .collect();

   let kdtree = kd_tree::KdTree::build(items.clone());

   let mut nn_edges: Vec<(i64, Edge, usize)> = items.iter()
      .map(|&p| {
         let mut nearest = kdtree.nearests(&p, 2);
         nearest.sort_by_key(|item| std::cmp::Reverse(item.squared_distance));
         let q: kd_tree::ItemAndDistance<'_, [i64; 3], i64> = nearest.get(0).expect("Could not find nearest neighbor!").clone();
         let edge = Edge::new(p, q.item.clone());
         (q.squared_distance, edge, 2)
      })
      .collect();

   nn_edges.sort_by_key(|(dist_sq, _, _)| std::cmp::Reverse(*dist_sq));

   let mut edges: HashSet<Edge> = HashSet::new();
   while edges.len() < num_edges {
      if let Some(val) = nn_edges.pop() {
         // Find N-th nearest neighbor to first point in edge and create a new edge
         let mut nearest = kdtree.nearests(&val.1.a, val.2 + 1);
         nearest.sort_by_key(|item| std::cmp::Reverse(item.squared_distance));           
         let q = nearest.get(0).expect("Could not find nearest neighbor!").clone();
         let edge = Edge::new(val.1.a.clone(), q.item.clone());

         // binary search insert into nn_edges
         let insert_pos = nn_edges.binary_search_by(|probe: &(i64, Edge, usize)| probe.0.cmp(&q.squared_distance).reverse()).unwrap_or_else(|i| i);
         nn_edges.insert(insert_pos, (q.squared_distance, edge, val.2+1));

         edges.insert(val.1);
      } else {
         panic!("Cannot not pop from nearest neigbor edges!");
      }
   }

   let mut components: Vec<HashSet<[i64;3]>> = vec![];
   for e in edges {
      // Find which existing components contain each endpoint
      let idx_a = components.iter().position(|pool| pool.contains(&e.a));
      let idx_b = components.iter().position(|pool| pool.contains(&e.b));

      match (idx_a, idx_b) {
         (Some(ia), Some(ib)) if ia == ib => {
               // Both already in the same component, nothing to do
         }
         (Some(ia), Some(ib)) => {
               // Merge ib into ia, then remove ib
               let pool_b = components.remove(ib);
               // After remove, ia may have shifted if ib < ia
               let ia = if ib < ia { ia - 1 } else { ia };
               components[ia].extend(pool_b);
         }
         (Some(ia), None) => {
               components[ia].insert(e.b);
         }
         (None, Some(ib)) => {
               components[ib].insert(e.a);
         }
         (None, None) => {
               // Neither endpoint seen yet; create a new component
               let new_pool: HashSet<[i64; 3]> = [e.a, e.b].into_iter().collect();
               components.push(new_pool);
         }
      }
   }

   components.sort_by_key(|component| std::cmp::Reverse(component.len()));
   let components: Vec<HashSet<[i64; 3]>> = components.into_iter().take(num_components).collect();

   components.iter()
      .map(|component| {
         component.len() as i64
      })
      .collect()
}

fn main() {
   let input = match fs::exists("day_8/input.txt") {
      Ok(_) => fs::read_to_string("day_8/input.txt").expect("Could not read file!"),
      Err(_) => download_input("https://adventofcode.com/2025/day/8/input")
   };

   let junction_boxes: Vec<&str> = input.lines().collect();

   let num_edges = 1000;
   let num_components = 3;

   let components = largest_components(&junction_boxes, num_edges, num_components);
   let comp_product: i64 = components.iter().product();
   println!("The product of the size of the {num_components} components given selecting {num_edges} edge is {comp_product}.");
}

#[cfg(test)]
mod tests {
   use super::*;
   use indoc::indoc;

   // Pulled from Advent of Code day 8 example
   // https://adventofcode.com/2025/day/8
   const INPUT: &str = indoc!{"
      162,817,812
      57,618,57
      906,360,560
      592,479,940
      352,342,300
      466,668,158
      542,29,236
      431,825,988
      739,650,466
      52,470,668
      216,146,977
      819,987,18
      117,168,530
      805,96,715
      346,949,466
      970,615,88
      941,993,340
      862,61,35
      984,92,344
      425,690,689
   "};

   #[test]
   fn test_component_product() {
      let input: Vec<&str> = INPUT.lines().collect();
      let mut given_comps = [5, 4, 2];
      let given_product = 40;

      let mut comps = largest_components(&input, 10, 3);
      let product: i64 = comps.iter().product();

      comps.sort();
      given_comps.sort();

      assert_eq!(comps, given_comps);
      assert_eq!(product, given_product);
   }
}