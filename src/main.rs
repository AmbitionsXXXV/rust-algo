use rust_algorithm::sorting::quick_sort;

pub mod sorting;

fn main() {
  let mut arr = vec![3, 4, 1];
  quick_sort::quick_sort(&mut arr);
  print!("{:?}", arr);
}
