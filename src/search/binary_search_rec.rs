use std::cmp::Ordering;

/// 使用递归实现的二分查找算法。在有序数组中搜索目标元素，并返回其索引。
/// 如果找到目标元素，则返回 Some(index)，否则返回 None。
///
/// # 参数
///
/// * `list_of_items` - 一个按升序排列的数组切片。
/// * `target` - 要搜索的目标元素的值。
/// * `left` - 当前搜索范围的左边界（包含）所在的索引。
/// * `right` - 当前搜索范围的右边界（不包含）所在的索引。
///
/// # 返回值
///
/// 如果找到目标元素，则返回 Some(index)，表示目标元素的索引。
/// 如果未找到目标元素，则返回 None。
///
/// # 复杂度
///
/// 该算法的时间复杂度为 O(log n)，其中 n 是数组的长度。
/// 空间复杂度为 O(log n)。
///
/// # 示例
///
/// ```rust
/// let arr = vec![1, 2, 3, 4, 5];
/// let target = 3;
/// let result = binary_search_rec(&arr, &target, &0, &arr.len());
/// assert_eq!(result, Some(2));
/// ```
///
/// -------
///
/// Recursive implementation of binary search algorithm. Searches for the target element in a sorted array
/// and returns its index if found, or None otherwise.
///
/// # Arguments
///
/// * `list_of_items` - A slice of the sorted array.
/// * `target` - The value of the target element to search for.
/// * `left` - The index of the left boundary (inclusive) of the current search range.
/// * `right` - The index of the right boundary (exclusive) of the current search range.
///
/// # Returns
///
/// If the target element is found, Some(index) is returned, indicating the index of the target element.
/// If the target element is not found, None is returned.
///
/// # Complexity
///
/// The time complexity of this algorithm is O(log n), where n is the length of the array.
/// The space complexity is O(log n).
///
/// # Example
///
/// ```rust
/// let arr = vec![1, 2, 3, 4, 5];
/// let target = 3;
/// let result = binary_search_rec(&arr, &target, &0, &arr.len());
/// assert_eq!(result, Some(2));
/// ```
pub fn binary_search_rec<T: Ord>(
  list_of_items: &[T],
  target: &T,
  left: &usize,
  right: &usize,
) -> Option<usize> {
  // If the left boundary is greater than or equal to the right boundary, the search range is empty, so return None.
  // 如果左边界大于等于右边界，说明搜索范围为空，返回 None
  if left >= right {
    return None;
  }

  // Calculate the middle index. Using (left + right) / 2 directly may cause overflow, so we use left + (right - left) / 2.
  // 计算中间索引，使用(left + right) / 2 的方式可能会导致溢出，所以使用 left + (right - left) / 2
  let middle: usize = left + (right - left) / 2;

  // Use match to compare the target element with the middle element.
  // 使用 match 对目标元素与中间元素进行比较
  match target.cmp(&list_of_items[middle]) {
    // If the target element is less than the middle element, continue searching in the left half.
    // 如果目标元素小于中间元素，则在左半边搜索
    Ordering::Less => binary_search_rec(list_of_items, target, left, &middle),

    // If the target element is greater than the middle element, continue searching in the right half.
    // 如果目标元素大于中间元素，则在右半边搜索
    Ordering::Greater => binary_search_rec(list_of_items, target, &(middle + 1), right),

    // If the target element is equal to the middle element, the target is found, so return Some(middle).
    // 如果目标元素等于中间元素，则找到目标元素，返回 Some(middle)
    Ordering::Equal => Some(middle),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const LEFT: usize = 0;

  #[test]
  fn fail_empty_list() {
    let list_of_items = vec![];

    assert_eq!(
      binary_search_rec(&list_of_items, &1, &LEFT, &list_of_items.len()),
      None
    );
  }

  #[test]
  fn success_one_item() {
    let list_of_items = vec![30];

    assert_eq!(
      binary_search_rec(&list_of_items, &30, &LEFT, &list_of_items.len()),
      Some(0)
    );
  }

  #[test]
  fn success_search_strings() {
    let say_hello_list = vec!["hi", "olá", "salut"];
    let RIGHT = say_hello_list.len();

    assert_eq!(
      binary_search_rec(&say_hello_list, &"hi", &LEFT, &RIGHT),
      Some(0)
    );

    assert_eq!(
      binary_search_rec(&say_hello_list, &"salut", &LEFT, &RIGHT),
      Some(2)
    );
  }

  #[test]
  fn fail_search_strings() {
    let say_hello_list = vec!["hi", "olá", "salut"];

    for target in &["adiós", "你好"] {
      assert_eq!(
        binary_search_rec(&say_hello_list, target, &LEFT, &say_hello_list.len()),
        None
      );
    }
  }

  #[test]
  fn success_search_integers() {
    let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];

    for (index, target) in integers.iter().enumerate() {
      assert_eq!(
        binary_search_rec(&integers, target, &LEFT, &integers.len()),
        Some(index)
      )
    }
  }

  #[test]
  fn fail_search_integers() {
    let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];

    for target in &[100, 444, 336] {
      assert_eq!(
        binary_search_rec(&integers, target, &LEFT, &integers.len()),
        None
      );
    }
  }

  #[test]
  fn fail_search_unsorted_strings_list() {
    let unsorted_strings = vec!["salut", "olá", "hi"];

    for target in &["hi", "salut"] {
      assert_eq!(
        binary_search_rec(&unsorted_strings, target, &LEFT, &unsorted_strings.len()),
        None
      );
    }
  }

  #[test]
  fn fail_search_unsorted_integers_list() {
    let unsorted_integers = vec![90, 80, 70, 60, 50, 40, 30, 20, 10, 0];

    for target in &[0, 80, 90] {
      assert_eq!(
        binary_search_rec(&unsorted_integers, target, &LEFT, &unsorted_integers.len()),
        None
      );
    }
  }

  #[test]
  fn success_search_string_in_middle_of_unsorted_list() {
    let unsorted_strings = vec!["salut", "olá", "hi"];

    assert_eq!(
      binary_search_rec(&unsorted_strings, &"olá", &LEFT, &unsorted_strings.len()),
      Some(1)
    );
  }

  #[test]
  fn success_search_integer_in_middle_of_unsorted_list() {
    let unsorted_integers = vec![90, 80, 70];

    assert_eq!(
      binary_search_rec(&unsorted_integers, &80, &LEFT, &unsorted_integers.len()),
      Some(1)
    );
  }
}

fn main() {}
