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

   // Did not end up using my own norm_sq implementation (kd_tree provides that value with nearests())
   // However, this is the first time I've used Rust's zip to simultaneously iterate over two collections at once
   fn _norm_sq(&self) -> i64 {
      self.a.iter()
         .zip(self.b.iter())
         .map(|(x, y)| x * y)
         .sum()
   }
}

// Given two points, A and B, we want edge (A, B) to be equivalen to edge (B, A) so we can use HashSet to filter out duplicates
impl PartialEq for Edge {
   fn eq(&self, other: &Self) -> bool {
      (self.a == other.a && self.b == other.b) ||
      (self.a == other.b && self.b == other.a)
   }
}

// We also need to implement a custom to make the order of A and B in the edge irrelevant 
impl Hash for Edge {
   fn hash<H: Hasher>(&self, state: &mut H) {
      // Hash both orderings the same way by sorting the two endpoints
      let (first, second) = if self.a <= self.b {
         (self.a, self.b)
      } else {
         (self.b, self.a)
      };

      // Feed the endpoints into the hasher state (nothing is returned, but hasher state will change)
      first.hash(state);
      second.hash(state);
   }
}

// Just a convenience function to reuse the parsing and kd-tree building code for both parts of the problem
fn build_problem(junction_boxes: &[&str]) -> (kd_tree::KdTree<[i64; 3]>, Vec<(i64, Edge, usize)>) {
   // Convert each line containing a 3D coordinate into an [i64; 3]
   let items: Vec<[i64; 3]> = junction_boxes.iter()
      .map(|s| {
         s.split(",")
            .map(|x| x.parse::<i64>().expect("coordinate components are not valid integers!"))
            .collect::<Vec<_>>()
            .try_into().expect("Each line of input must contain three whitespace delimited integers!")
      })
      .collect();

   // Insert all the 3D points into a kd-tree
   let kdtree = kd_tree::KdTree::build(items.clone());

   // Determine the nearest neighbor for each 3D point
   // This vector will contain duplicate edges, e.g., Edge{A, B} and Edge{B, A} will be present 
   // This is necessary for later when we find the next nearest neighbor to both endpoints when we consume an edge
   let mut nn_edges: Vec<(i64, Edge, usize)>= items.iter()
      .map(|&p| {
         let nearest = kdtree.nearests(&p, 2);
         let q: kd_tree::ItemAndDistance<'_, [i64; 3], i64> = nearest.get(1).expect("Could not find nearest neighbor!").clone();
         let edge = Edge::new(p, q.item.clone());
         (q.squared_distance, edge, 2)
      })
      .collect();

   // Sort from greatest to smallest distance squared (we're going to pop off the back of the vector later to get shortest edge)
   nn_edges.sort_by_key(|(dist_sq, _, _)| std::cmp::Reverse(*dist_sq));

   (kdtree, nn_edges)
}

fn largest_components(junction_boxes: &[&str], num_edges: usize, num_components: usize) -> Vec<i64> {
   let (kdtree, mut nn_edges) = build_problem(junction_boxes);

   // The algorithm is as follows,
   // 1. Take the shortest known edge (pop off back of nn_edges, which is already sorted)
   // 2. Find the next nearest neighbor to Edge.A (call it C), which will replace the the popped edge in nn_edges as Edge{A,C}
   // 3. Perform a sorted insert of Edge{A,C} into nn_edges to maintain sorted order (binary_search is used here)
   // 4. Add the two points A,B from Edge{A,B} (which was popped off nn_edges in (1)) to an appropriate graph component
   // 5. Continue until we've found edges.len() == num_edges
   // 6. Sort the graph components by size, and select the num_components largest components, and return their sizes
   let mut edges: HashSet<Edge> = HashSet::new();
   let mut components: Vec<HashSet<[i64;3]>> = vec![];
   while edges.len() < num_edges {
      if let Some(val) = nn_edges.pop() {
         // Find N-th nearest neighbor to first point in edge and create a new edge
         let nearest = kdtree.nearests(&val.1.a, val.2 + 1);
         let q = nearest.get(val.2).expect("Could not find nearest neighbor!").clone();
         let edge = Edge::new(val.1.a.clone(), q.item.clone());

         // binary search insert into nn_edges
         let insert_pos = nn_edges.binary_search_by(|probe: &(i64, Edge, usize)| probe.0.cmp(&q.squared_distance).reverse()).unwrap_or_else(|i| i);
         nn_edges.insert(insert_pos, (q.squared_distance, edge, val.2+1));

         // Insert the two 3D points making up our popped edge (val.1) into an appropriate graph component
         // We will need to be careful about how we do this, since we may need to merge two pre-existing components together
         let e = val.1;
         if !edges.contains(&e) {
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

            edges.insert(e);
         }

      } else {
         panic!("Cannot not pop from nearest neigbor edges!");
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

fn last_pair(junction_boxes: &[&str]) -> [[i64; 3]; 2] {
   let (kdtree, mut nn_edges) = build_problem(junction_boxes);
   
   // Prefill components with a single component for each point (i.e., just the set of all points)
   // We need to do this since our condition for returning an answer hinges on the last two components being merged together
   let mut components: Vec<HashSet<[i64;3]>> = nn_edges.iter().map(|(_, edge, _)| HashSet::from([edge.a])).collect();

   // The algorithm is as follows,
   // 1. Take the shortest known edge (pop off back of nn_edges, which is already sorted)
   // 2. Find the next nearest neighbor to Edge.A (call it C), which will replace the the popped edge in nn_edges as Edge{A,C}
   // 3. Perform a sorted insert of Edge{A,C} into nn_edges to maintain sorted order (binary_search is used here)
   // 4. Add the two points A,B from Edge{A,B} (which was popped off nn_edges in (1)) to an appropriate graph component
   // 6. Loop infinitely and add a break when adding an edge will result in a merge of the last two components
   let mut edges: HashSet<Edge> = HashSet::new();
   loop {
      if let Some(val) = nn_edges.pop() {
         // Find N-th nearest neighbor to first point in edge and create a new edge
         let nearest = kdtree.nearests(&val.1.a, val.2 + 1);
         let q = nearest.get(val.2).expect("Could not find nearest neighbor!").clone();
         let edge = Edge::new(val.1.a.clone(), q.item.clone());

         // binary search insert into nn_edges
         let insert_pos = nn_edges.binary_search_by(|probe: &(i64, Edge, usize)| probe.0.cmp(&q.squared_distance).reverse()).unwrap_or_else(|i| i);
         nn_edges.insert(insert_pos, (q.squared_distance, edge, val.2+1));

         // Insert the two 3D points making up our popped edge (val.1) into an appropriate graph component
         // We will need to be careful about how we do this, since we may need to merge two pre-existing components together
         let e = val.1;
         if !edges.contains(&e) {
            // Find which existing components contain each endpoint
            let idx_a = components.iter().position(|pool| pool.contains(&e.a));
            let idx_b = components.iter().position(|pool| pool.contains(&e.b));

            match (idx_a, idx_b) {
               (Some(ia), Some(ib)) if ia == ib => {
                     // Both already in the same component, nothing to do
               }
               (Some(ia), Some(ib)) => {
                     // If we're about to merge the last two components, instead return the two points
                     // that make up the final edge that's going to be added to the graph
                     if components.len() == 2 {
                        return [e.a, e.b];
                     }

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

            edges.insert(e);
         }

      } else {
         panic!("Cannot not pop from nearest neigbor edges!");
      }
   }
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
   
   let pairs = last_pair(&junction_boxes);
   let x_product: i64 = pairs[0][0] * pairs[1][0];
   println!("The product of the X-coordinates of the last two junction boxes is {x_product}");
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

   #[test]
   fn test_last_pair() {
      let input: Vec<&str> = INPUT.lines().collect();
      let mut given_pair: [[i64; 3]; 2] = [[216,146,977], [117,168,530]];
      let given_x_product = 25272;

      let mut pairs = last_pair(&input);
      let x_product: i64 = pairs[0][0] * pairs[1][0];

      given_pair.sort();
      pairs.sort();

      assert_eq!(pairs, given_pair);
      assert_eq!(x_product, given_x_product);
   }
}