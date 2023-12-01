pub mod kth_smallest {
  use std::cmp::Ordering;

  use rust_algorithm::sorting::quick_sort::partition;

  /// 计算给定可变切片中的第 k 小元素。
  ///
  /// Uses the QuickSelect algorithm to find the kth smallest element. This function
  /// modifies the input slice for sorting purposes, but does not perform element copying.
  /// If the input slice is empty or k is out of range, it returns None.
  ///
  /// 使用快速选择算法来查找第 k 小元素。该函数会修改输入切片以进行排序，但不会
  /// 对切片中的元素进行拷贝。如果输入切片为空或 k 超出了切片范围，将返回 None。
  ///
  /// # 参数 (Parameters)
  /// - `input`: 可变切片，包含待查找第 k 小元素的元素。 (Mutable slice containing elements to find the kth smallest element from.)
  /// - `k`: 要查找的第 k 小元素的位置。 (The position of the kth smallest element to find.)
  ///
  /// # 返回值 (Returns)
  /// - 如果找到第 k 小元素，则返回 Some(T)，其中 T 是元素的类型。
  /// - If the kth smallest element is found, it returns Some(T), where T is the type of the element.
  /// - 如果输入切片为空或 k 超出了切片范围，则返回 None。
  /// - If the input slice is empty or k is out of range, it returns None.
  ///
  /// # 使用示例 (Example)
  /// ```
  /// let mut nums = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
  /// let k = 5;
  /// let result = kth_smallest(&mut nums, k);
  /// assert_eq!(result, Some(3));
  /// ```
  ///
  /// # 算法复杂度 (Algorithm Complexity)
  /// - 平均情况下，时间复杂度为 O(n)，其中 n 是切片的大小。 (Average-case time complexity is O(n), where n is the size of the slice.)
  /// - 最坏情况下，时间复杂度为 O(n^2)。但通过选择合适的主元素和随机化，最坏情况
  ///   可以避免。 (Worst-case time complexity is O(n^2). However, this can be mitigated by selecting
  ///   a suitable pivot element and applying randomization.)
  ///
  /// # 注意事项 (Note)
  /// - 这个函数会修改输入切片以进行排序。 (This function modifies the input slice for sorting purposes.)
  pub fn kth_smallest<T>(input: &mut [T], k: usize) -> Option<T>
  where
    T: PartialOrd + Copy,
  {
    // 如果输入数组为空，直接返回None
    // If the input array is empty, return None.
    if input.is_empty() {
      return None;
    }

    // 调用私有函数_kth_smallest来找到第k小的元素
    // Call the private function _kth_smallest to find the kth smallest element.
    let kth = _kth_smallest(input, k, 0, input.len() - 1);
    Some(kth)
  }

  fn _kth_smallest<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T
  where
    T: PartialOrd + Copy,
  {
    // 如果子数组只有一个元素，直接返回该元素
    // If the subarray contains only one element, return that element.
    if lo == hi {
      return input[lo];
    }

    // 使用partition函数找到pivot元素的索引
    // Use the partition function to find the index of the pivot element.
    let pivot = partition(input, lo, hi);
    // 计算pivot元素是第几小的元素（i）
    // Calculate the position (i) of the pivot element.
    let i = pivot - lo + 1;

    // 根据k和i的比较来决定递归查找左边或右边的子数组
    // Determine whether to recursively search in the left or right subarray based on the comparison of k and i.
    match k.cmp(&i) {
      // 如果k等于i，直接返回pivot元素
      // If k is equal to i, directly return the pivot element.
      Ordering::Equal => input[pivot],
      // 如果k小于i，递归在左边子数组中查找
      // If k is less than i, recursively search in the left subarray.
      Ordering::Less => _kth_smallest(input, k, lo, pivot - 1),
      // 如果k大于i，递归在右边子数组中查找
      // If k is greater than i, recursively search in the right subarray.
      Ordering::Greater => _kth_smallest(input, k - i, pivot + 1, hi),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::kth_smallest::kth_smallest;

  #[test]
  fn empty() {
    let mut zero: [u8; 0] = [];
    let first = kth_smallest(&mut zero, 1);
    assert_eq!(None, first);
  }

  #[test]
  fn one_element() {
    let mut one = [1];
    let first = kth_smallest(&mut one, 1);
    assert_eq!(1, first.unwrap());
  }

  #[test]
  fn many_elements() {
    // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
    let mut many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];
    let first = kth_smallest(&mut many, 1);
    let third = kth_smallest(&mut many, 3);
    let sixth = kth_smallest(&mut many, 6);
    let fourteenth = kth_smallest(&mut many, 14);
    assert_eq!(0, first.unwrap());
    assert_eq!(3, third.unwrap());
    assert_eq!(7, sixth.unwrap());
    assert_eq!(17, fourteenth.unwrap());
  }
}

fn main() {}
