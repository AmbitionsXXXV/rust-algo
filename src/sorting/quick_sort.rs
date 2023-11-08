use rand::Rng;

pub fn main() {}

/// 使用快速排序算法对可变切片进行升序排序。
///
/// Sorts a mutable slice in ascending order using the Quick Sort algorithm.
///
/// # 示例 (Examples)
///
/// ```
/// let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// quick_sort(&mut numbers);
/// assert_eq!(numbers, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
///
/// # 参数 (Arguments)
///
/// * `arr`: 待排序的可变切片的引用。 (A mutable reference to the slice to be sorted.)
///
pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
  if arr.len() > 1 {
    quick_sort_range(arr, 0, arr.len() - 1);
  }
}

/// 递归地使用快速排序算法对可变切片的指定范围进行升序排序。
///
/// Recursively sorts a range of a mutable slice in ascending order using the Quick Sort algorithm.
///
/// # 参数 (Arguments)
///
/// * `arr`: 待排序的可变切片的引用。 (A mutable reference to the slice to be sorted.)
/// * `lo`: 排序范围的低索引。 (The low index of the range to be sorted.)
/// * `hi`: 排序范围的高索引。 (The high index of the range to be sorted.)
///
fn quick_sort_range<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
  if lo < hi {
    let pos = partition(arr, lo, hi);

    if pos != 0 {
      quick_sort_range(arr, lo, pos.wrapping_sub(1)); // 使用 wrapping_sub 避免下溢 (Using wrapping_sub to avoid underflow)
    }
    quick_sort_range(arr, pos + 1, hi);
  }
}

/// 辅助函数，使用快速选择算法查找第 k 小元素。
///
/// Helper function that finds the kth smallest element using the QuickSelect algorithm.
/// This function searches for the kth smallest element within the given slice in the
/// range from lo to hi.
///
/// 这个函数在给定的切片中查找第 k 小元素，从 lo 到 hi 的范围内进行查找。
///
/// # 参数 (Parameters)
/// - `input`: 可变切片，包含待查找第 k 小元素的元素。 (Mutable slice containing elements to find the kth smallest element from.)
/// - `k`: 要查找的第 k 小元素的位置。 (The position of the kth smallest element to find.)
/// - `lo`: 当前查找范围的起始索引。 (The starting index of the current search range.)
/// - `hi`: 当前查找范围的结束索引。 (The ending index of the current search range.)
///
/// # 返回值 (Returns)
/// - 找到第 k 小元素时，返回元素的值。 (Returns the value of the kth smallest element when found.)
///
/// # 算法复杂度 (Algorithm Complexity)
/// - 平均情况下，时间复杂度为 O(n)，其中 n 是切片的大小。 (Average-case time complexity is O(n), where n is the size of the slice.)
/// - 最坏情况下，时间复杂度为 O(n^2)。但通过选择合适的主元素和随机化，最坏情况
///   可以避免。 (Worst-case time complexity is O(n^2). However, this can be mitigated by selecting
///   a suitable pivot element and applying randomization.)
///
/// # 注意事项 (Note)
/// - 这个函数会修改输入切片以进行排序。 (This function modifies the input slice for sorting purposes.)
///
/// # 复杂度 (Complexity)
/// Omitted detailed comments about the partitioning implementation as it is not the primary focus of this function.
/// This function performs the partitioning operation for the QuickSelect algorithm.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
  // 默认选择 `lo` 作为基准 (Default to choosing `lo` as the pivot)
  let pivot = lo;
  let partition_pos = partition_helper(arr, pivot, lo, hi);
  arr.swap(pivot, partition_pos);
  partition_pos
}

/// 将可变切片在随机基准元素周围进行分区，确保左侧元素较小，右侧元素较大。
///
/// Partitions a mutable slice randomly around a pivot element, ensuring elements on the left are smaller and elements on the right are larger.
///
/// # 参数 (Arguments)
///
/// * `arr`: 待分区的可变切片的引用。 (A mutable reference to the slice to be partitioned.)
/// * `lo`: 分区范围的低索引。 (The low index of the range to be partitioned.)
/// * `hi`: 分区范围的高索引。 (The high index of the range to be partitioned.)
///
/// # 返回 (Returns)
///
/// 分区完成后基准元素的最终索引。 (The final index of the pivot element after partitioning.)
///
fn partition_random<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
  let mut rng = rand::thread_rng();
  let pivot = rng.gen_range(lo..=hi);

  arr.swap(lo, pivot);
  let partition_pos = partition_helper(arr, lo, lo, hi);
  arr.swap(lo, partition_pos);
  partition_pos
}

/// 辅助函数，将可变切片围绕基准元素进行分区。
///
/// Helper function for partitioning a mutable slice around a pivot element.
///
/// # 参数 (Arguments)
///
/// * `arr`: 待分区的可变切片的引用。 (A mutable reference to the slice to be partitioned.)
/// * `pivot`: 基准元素的索引。 (The index of the pivot element.)
/// * `lo`: 分区范围的低索引。 (The low index of the range to be partitioned.)
/// * `hi`: 分区范围的高索引。 (The high index of the range to be partitioned.)
///
/// # 返回 (Returns)
///
/// 分区完成后基准元素的最终索引。 (The final index of the pivot element after partitioning.)
///
fn partition_helper<T: PartialOrd>(arr: &mut [T], pivot: usize, lo: usize, hi: usize) -> usize {
  let mut left = lo;
  let mut right = hi;

  while left < right {
    while left < right && arr[right] >= arr[pivot] {
      right -= 1;
    }
    while left < right && arr[left] <= arr[pivot] {
      left += 1;
    }
    if left != right {
      arr.swap(left, right);
    }
  }

  left
}

#[cfg(test)]
mod tests {
  use super::quick_sort;

  #[test]
  fn test_empty_vec() {
    let mut empty_vec: Vec<String> = vec![];
    quick_sort(&mut empty_vec);
    assert_eq!(empty_vec, Vec::<String>::new());
  }

  #[test]
  fn test_number_vec() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    quick_sort(&mut vec);
    assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
  }

  #[test]
  fn test_string_vec() {
    let mut vec = vec![
      String::from("Bob"),
      String::from("David"),
      String::from("Carol"),
      String::from("Alice"),
    ];
    quick_sort(&mut vec);
    assert_eq!(
      vec,
      vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Carol"),
        String::from("David"),
      ]
    );
  }
}
