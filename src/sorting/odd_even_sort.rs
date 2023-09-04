pub mod odd_even_sort {
  /// Sorts a mutable slice of elements using the odd-even sort algorithm.
  ///
  /// Odd-even sort is a simple comparison-based sorting algorithm that works by comparing and
  /// swapping adjacent elements at odd and even positions. It repeats this process until the
  /// entire array is sorted.
  ///
  /// 使用奇偶排序算法对可变切片的元素进行排序。
  ///
  /// 奇偶排序是一种简单的基于比较的排序算法，它通过比较和交换奇数和偶数位置上的相邻元素来工作。
  /// 它重复这个过程，直到整个数组被排序。
  ///
  /// # Examples
  ///
  /// ```
  /// let mut numbers = vec![9, 4, 2, 7, 5, 8, 1, 6, 3];
  /// odd_even_sort(&mut numbers);
  /// assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  /// ```
  ///
  /// # Arguments
  ///
  /// * `arr` - A mutable reference to a slice of elements.
  ///
  /// # Generic Constraints
  ///
  /// The generic type parameter `T` is defined with the `Ord` trait constraint to indicate that it
  /// must support comparison for sorting.
  ///
  /// 泛型类型参数 `T` 使用 `Ord` trait 约束来表示它必须支持比较以进行排序。
  pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len == 0 {
      return;
    }

    let mut sorted = false;

    while !sorted {
      sorted = true;

      // Compare and swap elements at odd positions
      // 比较和交换奇数位上的元素
      for i in (1..len - 1).step_by(2) {
        if arr[i] > arr[i + 1] {
          arr.swap(i, i + 1);
          sorted = false; // Set to false to indicate potential swaps
        }
      }

      // Compare and swap elements at even positions
      // 比较和交换偶数位上的元素
      for i in (0..len - 1).step_by(2) {
        if arr[i] > arr[i + 1] {
          arr.swap(i, i + 1);
          sorted = false; // Set to false to indicate potential swaps
        }
      }
    }
  }
}

pub fn main() {}

#[cfg(test)]
mod tests {
  use crate::odd_even_sort::odd_even_sort;

  #[test]
  fn basic() {
    let mut arr = vec![3, 5, 1, 2, 4, 6];
    odd_even_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
  }

  #[test]
  fn empty() {
    let mut arr = Vec::<i32>::new();
    odd_even_sort(&mut arr);
    assert_eq!(arr, vec![]);
  }

  #[test]
  fn one_element() {
    let mut arr = vec![3];
    odd_even_sort(&mut arr);
    assert_eq!(arr, vec![3]);
  }

  #[test]
  fn pre_sorted() {
    let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    odd_even_sort(&mut arr);
    assert_eq!(arr, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
  }
}
