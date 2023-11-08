/// 归并排序是一种常见的排序算法，它采用分治的思想实现。具体步骤如下：
///
/// 1. 分解：将待排序的数组分解成两个子数组，取中间位置将数组分为左右两部分
/// 2. 解决：递归地对左右两个子数组进行排序，直到子数组的长度为 1 或 0（已经有序）
/// 3. 合并：将排好序的两个子数组合并成一个有序数组
///
/// # Arguments
///
/// * `arr`:
///
/// returns: ()
/// This function performs the merge sort algorithm on a mutable slice of comparable elements.
///
/// Merge sort is a divide-and-conquer sorting algorithm that recursively divides the array into
/// smaller subarrays, sorts them, and then merges the sorted subarrays back together.
///
/// 归并排序是一种分治排序算法，它将数组递归地划分为较小的子数组，对它们进行排序，然后合并排序后的子数组。
///
/// # Examples
///
/// ```
/// let mut numbers = vec![9, 4, 2, 7, 5, 8, 1, 6, 3];
/// merge_sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
///
/// # Arguments
///
/// * `arr` - A mutable reference to a slice of comparable elements.
///
/// # Generic Constraints
///
/// The generic type parameter `T` is defined with trait bounds to indicate that it must support
/// partial ordering (`PartialOrd`), cloning (`Clone`), and have a default value (`Default`).
///
/// 泛型类型参数 `T` 使用 trait 约束来表示它必须支持部分排序（`PartialOrd`）、克隆（`Clone`）
/// 并具有默认值（`Default`）。
pub fn merge_sort<T>(arr: &mut [T])
where
  T: PartialOrd + Clone + Default,
{
  if arr.len() > 1 {
    merge_sort_range(arr, 0, arr.len() - 1);
  }
}

/// Recursively sorts a range of elements within the array using merge sort.
///
/// This function divides the array range into halves, sorts each half, and then merges the sorted halves
/// back together.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the entire array.
/// * `lo` - The lower index of the range to sort.
/// * `hi` - The higher index of the range to sort.
///
/// 递归地对数组中的一系列元素执行归并排序。
///
/// 该函数将数组范围划分为两半，分别对每一半进行排序，然后将排序后的两半合并回来。
///
/// # 参数
///
/// * `arr` - 对整个数组的可变引用。
/// * `lo` - 要排序范围的下限索引。
/// * `hi` - 要排序范围的上限索引。
fn merge_sort_range<T>(arr: &mut [T], lo: usize, hi: usize)
where
  T: PartialOrd + Clone + Default,
{
  // Only perform sorting when there are more than one elements
  // 只有在元素数量大于 1 时才执行排序
  if lo < hi {
    // 当前子数组的中间索引
    let mid = lo + ((hi - lo) >> 1);

    merge_sort_range(arr, lo, mid);
    merge_sort_range(arr, mid + 1, hi);
    merge_two_arrays(arr, lo, mid, hi);
  }
}

/// Merges two sorted arrays within the specified range.
///
/// This function takes two sorted arrays and merges them into a single sorted array within the given range.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the entire array.
/// * `lo` - The lower index of the first sorted array.
/// * `mid` - The higher index of the first sorted array and the lower index of the second sorted array.
/// * `hi` - The higher index of the second sorted array.
///
/// 在指定范围内合并两个已排序数组。
///
/// 该函数接受两个已排序数组，并将它们合并为一个已排序数组，范围在给定范围内。
///
/// # 参数
///
/// * `arr` - 对整个数组的可变引用。
/// * `lo` - 第一个已排序数组的下限索引。
/// * `mid` - 第一个已排序数组的上限索引和第二个已排序数组的下限索引。
/// * `hi` - 第二个已排序数组的上限索引。
fn merge_two_arrays<T>(arr: &mut [T], lo: usize, mid: usize, hi: usize)
where
  T: PartialOrd + Clone + Default,
{
  // Clone elements into temporary arrays
  // 克隆元素到临时数组
  let mut arr1 = arr[lo..=mid].to_vec();
  let mut arr2 = arr[mid + 1..=hi].to_vec();
  let (mut i, mut j) = (0, 0);

  // Merge the two arrays back into the main array
  // 将两个数组合并回主数组
  while i < arr1.len() && j < arr2.len() {
    if arr1[i] < arr2[j] {
      arr[i + j + lo] = std::mem::take(&mut arr1[i]);
      i += 1;
    } else {
      arr[i + j + lo] = std::mem::take(&mut arr2[j]);
      j += 1;
    }
  }

  // Append any remaining elements from the first array
  // 从第一个数组追加剩余元素
  while i < arr1.len() {
    arr[i + j + lo] = std::mem::take(&mut arr1[i]);
    i += 1;
  }

  // Append any remaining elements from the second array
  // 从第二个数组追加剩余元素
  while j < arr2.len() {
    arr[i + j + lo] = std::mem::take(&mut arr2[j]);
    j += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::merge_sort;

  #[test]
  fn test_empty_vec() {
    let mut empty_vec: Vec<String> = vec![];

    merge_sort(&mut empty_vec);

    assert_eq!(empty_vec, Vec::<String>::new());
  }

  #[test]
  fn test_number_vec() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];

    merge_sort(&mut vec);

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

    merge_sort(&mut vec);

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

fn main() {}
