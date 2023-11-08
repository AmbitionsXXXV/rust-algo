use std::ops::AddAssign;

fn main() {}

// 时间复杂度 O(n + maxval)
// 空间复杂度 O(maxval)
pub fn counting_sort(arr: &mut [u32], maxval: usize) {
  let mut occurrences: Vec<usize> = vec![0; maxval + 1];

  for &data in arr.iter() {
    occurrences[data as usize] += 1;
  }

  let mut i = 0;
  for (data, &number) in occurrences.iter().enumerate() {
    for _ in 0..number {
      arr[i] = data as u32;
      i += 1;
    }
  }
}

pub fn generic_counting_sort<T: Into<u64> + From<u8> + AddAssign + Copy>(
  arr: &mut [T],
  maxval: usize,
) {
  let mut occurrences: Vec<usize> = vec![0; maxval + 1];

  for &data in arr.iter() {
    occurrences[data.into() as usize] += 1;
  }

  // Current index in output array
  let mut i = 0;
  // current data point, necessary to be type-safe
  let mut data = T::from(0);
  // This will iterate from 0 to the largest data point in `arr`
  // `number` contains the occurrences of the data point `data`
  for &number in occurrences.iter() {
    for _ in 0..number {
      arr[i] = data;
      i += 1;
    }

    data += T::from(1);
  }
}

pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
  arr.windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod test {
  use super::{counting_sort, generic_counting_sort, is_sorted};

  #[test]
  fn counting_sort_descending() {
    let mut ve1 = vec![6, 5, 4, 3, 2, 1];
    counting_sort(&mut ve1, 6);
    assert!(is_sorted(&ve1));
  }

  #[test]
  fn counting_sort_pre_sorted() {
    let mut ve2 = vec![1, 2, 3, 4, 5, 6];
    counting_sort(&mut ve2, 6);
    assert!(is_sorted(&ve2));
  }

  #[test]
  fn presorted_u64_counting_sort() {
    let mut ve2: Vec<u64> = vec![1, 2, 3, 4, 5, 6];
    generic_counting_sort(&mut ve2, 6);
    assert!(is_sorted(&ve2));
  }
}
