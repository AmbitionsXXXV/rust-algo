fn main() {}

/// 对可变切片 `arr` 使用基数排序（radix sort）进行就地排序。
///
/// # 算法描述 (Algorithm description)
///
/// 基数排序（Radix Sort）是一种非比较型的排序算法，它根据数字的位数将数字按照从低位到高位的顺序进行排序。
/// 基数排序的思想是：首先按照最低位对所有数字进行排序，然后再按照次低位进行排序，依此类推，直到最高位。
///
/// 基数排序的时间复杂度取决于数字的位数、基数和元素的数量。当位数不是很大，且基数和元素数量相当时，基数排序可以达到线性时间复杂度。
///
/// # 算法复杂度 (Algorithm complexity)
///
/// 时间复杂度为 `O((n + b) * logb(k))`，其中 `n` 是元素个数，
/// `b` 是基数，`k` 是最大元素。当 `n` 和 `b` 大致相同时，该算法具有线性时间复杂度。
///
/// 空间复杂度为 `O(n + b)`。
///
/// Time complexity is `O((n + b) * logb(k))`, where `n` is the number of elements,
/// `b` is the base (the radix), and `k` is the largest element.
/// When `n` and `b` are roughly the same maginitude, this algorithm runs in linear time.
///
/// Space complexity is `O(n + b)`.
///
/// # 参数 (Arguments)
///
/// * `arr`: 待排序的可变切片的引用。 (A mutable reference to the slice to be sorted.)
///
pub fn radix_sort(arr: &mut [u64]) {
  // 找到数组中的最大值，以确定排序的次数 (Find the maximum value in the array to determine the number of sorts)
  let max: usize = match arr.iter().max() {
    Some(&x) => x as usize,
    None => return,
  };

  // 选择一个接近数组长度的2的幂作为基数，以优化运行时间 (Choose a power of 2 close to array length as radix for optimal runtime)
  let radix = arr.len().next_power_of_two();
  // 从最低有效位到最高有效位逐位进行计数排序 (Counting sort by each digit from least to most significant)
  let mut place = 1;

  while place <= max {
    // 获取数字的某位数 (Get the digit at a certain place)
    let digit_of = |x| x as usize / place % radix;
    // 计算每个位上数字出现的次数 (Count digit occurrences)
    let mut counter = vec![0; radix];

    for &x in arr.iter() {
      counter[digit_of(x)] += 1;
    }

    // 计算每个位上数字的最后一个索引 (Compute last index of each digit)
    for i in 1..radix {
      counter[i] += counter[i - 1];
    }

    // 将元素按位重新排序到正确的位置 (Write elements to their new indices)
    let arr_copy = arr.to_owned(); // 为了在迭代过程中避免 borrow 错误 (To avoid borrow errors during iteration)
    for &x in arr_copy.iter().rev() {
      counter[digit_of(x)] -= 1;
      arr[counter[digit_of(x)]] = x;
    }

    // 转到下一位 (Move to the next place)
    place *= radix;
  }
}

#[cfg(test)]
mod tests {
  use super::radix_sort;
  use crate::sorting::counting_sort::is_sorted;

  #[test]
  fn empty() {
    let mut a: [u64; 0] = [];
    radix_sort(&mut a);
    assert!(is_sorted(&a));
  }

  #[test]
  fn descending() {
    let mut v = vec![201, 127, 64, 37, 24, 4, 1];
    radix_sort(&mut v);
    assert!(is_sorted(&v));
  }

  #[test]
  fn ascending() {
    let mut v = vec![1, 4, 24, 37, 64, 127, 201];
    radix_sort(&mut v);
    assert!(is_sorted(&v));
  }
}
