use std::cmp::Ordering;

/// 该函数实现了二分查找算法，用于在给定的有序数组中查找给定元素的位置。
/// This function implements the binary search algorithm to find the position of a given element in a given sorted array.
///
/// # Arguments
///
/// * `item` - 待查找的元素
///   The element to be searched for.
/// * `arr` - 给定的有序数组
///   The given sorted array.
///
/// # Returns
///
/// 返回找到的元素在数组中的索引值。如果未找到，则返回 None。
/// Returns the index of the found element in the array. If not found, returns None.
///
/// # Examples
///
/// ```
/// let items = vec![1, 2, 3, 4, 5];
/// assert_eq!(binary_search(&3, &items), Some(2));
///
/// let items = vec!['a', 'b', 'c', 'd'];
/// assert_eq!(binary_search(&'c', &items), Some(2));
///
/// let items: [i32; 0] = [];
/// assert_eq!(binary_search(&1, &items), None);
/// ```
///
/// # Time Complexity
///
/// 在最坏情况下，时间复杂度为 O(log n)，其中 n 是数组的长度。
/// The time complexity of this function in the worst case is O(log n), where n is the length of the array.
///
/// # Space Complexity
///
/// 空间复杂度为 O(1)。
/// The space complexity is O(1).
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
  // 设置左边界为数组起始位置
  // set the left boundary to the start of the array
  let mut left = 0;
  // 设置右边界为数组长度
  // set the right boundary to the length of the array
  let mut right = arr.len();

  // 当左边界小于右边界时进行循环
  // loop until the left boundary is greater than or equal to the right boundary
  while left < right {
    // 计算当前区间的中间位置
    // calculate the midpoint of the current search range
    let mid = left + (right - left) / 2;

    // 使用三路比较法比较目标值和中间位置处的元素
    // use a three-way comparison to compare the item with the element at the midpoint
    match item.cmp(&arr[mid]) {
      // 如果目标值小于中间位置处的元素，则将右边界更新为中间位置，缩小搜索范围至左半部分
      // if the item is less than the midpoint element, narrow down the search range to the left half
      Ordering::Less => right = mid,

      // 如果目标值等于中间位置处的元素，则返回该位置索引，表示找到目标元素
      // if the item is equal to the midpoint element, return the index of the element to indicate it has been found
      Ordering::Equal => return Some(mid),

      // 如果目标值大于中间位置处的元素，则将左边界更新为中间位置加一，缩小搜索范围至右半部分
      // if the item is greater than the midpoint element, narrow down the search range to the right half
      Ordering::Greater => left = mid + 1,
    }
  }

  // 循环结束后仍未找到目标元素，返回None表示未找到
  // if the loop completes without finding the element, return None to indicate it was not found in the array
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty() {
    let index = binary_search(&"a", &vec![]);

    assert_eq!(index, None);
  }

  #[test]
  fn one_item() {
    let index = binary_search(&"a", &vec!["a"]);

    assert_eq!(index, Some(0));
  }

  #[test]
  fn search_strings() {
    let index = binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);

    assert_eq!(index, Some(0));
  }

  #[test]
  fn search_ints() {
    let index = binary_search(&4, &vec![1, 2, 3, 4]);

    assert_eq!(index, Some(3));

    let index = binary_search(&3, &vec![1, 2, 3, 4]);

    assert_eq!(index, Some(2));

    let index = binary_search(&2, &vec![1, 2, 3, 4]);

    assert_eq!(index, Some(1));

    let index = binary_search(&1, &vec![1, 2, 3, 4]);

    assert_eq!(index, Some(0));
  }

  #[test]
  fn not_found() {
    let index = binary_search(&5, &vec![1, 2, 3, 4]);

    assert_eq!(index, None);
  }
}

fn main() {}
