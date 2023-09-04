pub mod tim_sort {
  use std::cmp::min;

  /// Find the minimum run length suitable for Tim Sort, calculated based on the length of the array.
  ///
  /// # Parameters
  ///
  /// * `n`: Length of the array.
  /// Returns the minimum run length.
  pub fn find_min_run(mut n: usize) -> usize {
    let mut r = 0;
    let minimum = 64;

    // 计算最小的 run 长度
    // Calculate the minimum run length
    while n >= minimum {
      r |= n & 1;
      n >>= 1;
    }

    n + r
  }

  /// Perform insertion sort within a specified range of the array.
  ///
  /// # Parameters
  ///
  /// * `arr`: Mutable reference to the array to be sorted.
  /// * `left`: Starting index of the range.
  /// * `right`: Ending index of the range.
  pub fn insert_sort<T>(arr: &mut [T], left: usize, right: usize)
  where
    T: PartialOrd + Copy,
  {
    // 在指定的范围内执行插入排序
    // Iterate through the range and insert elements in sorted order
    for i in left + 1..right + 1 {
      let element = arr[i];
      let mut j = i - 1;

      // 使用插入排序找到当前元素的正确位置
      // Find the correct position for the current element using insertion sort
      while element < arr[j] && j >= left {
        arr[j + 1] = arr[j];
        j -= 1;
      }

      arr[j + 1] = element;
    }
  }

  /// Merge two sorted subarrays of an array.
  ///
  /// # Parameters
  ///
  /// * `arr`: Mutable reference to the array.
  /// * `left`: Starting index of the left subarray.
  /// * `mid`: Ending index of the left subarray and starting index of the right subarray.
  /// * `right`: Ending index of the right subarray.
  pub fn merge<T>(arr: &mut [T], left: usize, mid: usize, right: usize)
  where
    T: PartialOrd + Copy,
  {
    // 创建左右子数组的临时向量
    // Create temporary vectors for the left and right subarrays
    let array_length1 = mid - left + 1;
    let array_length2 = right - mid;
    let left_arr = Vec::from(&arr[left..left + array_length1]);
    let right_arr = Vec::from(&arr[mid + 1..mid + 1 + array_length2]);
    let (mut i, mut j, mut k) = (0, 0, left);

    // 将左右子数组合并回主数组
    // Merge the left and right subarrays back into the main array
    while j < array_length2 && i < array_length1 {
      // If the element in the left subarray is smaller or equal, copy it to the main array
      if left_arr[i] <= right_arr[j] {
        arr[k] = left_arr[i];
        i += 1;
      } else {
        // If the element in the right subarray is smaller, copy it to the main array
        arr[k] = right_arr[j];
        j += 1;
      }

      k += 1;
    }

    // 将剩余的元素从左右子数组复制回主数组
    // Copy any remaining elements from the left and right subarrays back into the main array
    while i < array_length1 {
      arr[k] = left_arr[i];
      k += 1;
      i += 1;
    }

    while j < array_length2 {
      arr[k] = right_arr[j];
      k += 1;
      j += 1;
    }
  }

  /// Perform Tim Sort on an array.
  ///
  /// # Parameters
  ///
  /// * `arr`: Mutable reference to the array to be sorted.
  pub fn tim_sort<T>(arr: &mut [T])
  where
    T: PartialOrd + Copy,
  {
    let n = arr.len();
    let min_run = find_min_run(n);

    // 对每个最小长度的 run 执行插入排序
    // Insertion sort each run of minimum length
    for start in (0..n).filter(|x| x % min_run == 0) {
      let end = min(start + min_run - 1, n - 1);

      insert_sort(arr, start, end)
    }

    let mut size = min_run;

    // 合并相邻的 run，直到整个数组排序完成
    // Merge adjacent runs until the entire array is sorted
    while size < n {
      // 合并相邻的 run
      // Merge adjacent runs
      for left in (0..n).filter(|x| x % (2 * size) == 0) {
        // 计算中间索引和右侧索引
        // Calculate the middle index and the right index
        let mid = min(n - 1, left + size - 1);
        let right = min(left + 2 * size - 1, n - 1);

        // 合并两个子数组
        // Merge the two subarrays
        merge(arr, left, mid, right)
      }

      // 增大 run 的大小，以便继续合并相邻的 run
      // Increase the size of the run for the next iteration
      size = 2 * size;
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tim_sort::{find_min_run, insert_sort, tim_sort};

  #[test]
  fn cal_min() {
    let n = 189;
    let min_run = find_min_run(n);

    assert_eq!(48, min_run);
    assert_eq!(61, find_min_run(976));
  }

  #[test]
  fn insert_test() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];

    insert_sort(&mut vec, 0, 9);

    assert_eq!(vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78], vec)
  }

  #[test]
  fn tim_test() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];

    tim_sort(&mut vec);

    assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
  }
}

fn main() {}
