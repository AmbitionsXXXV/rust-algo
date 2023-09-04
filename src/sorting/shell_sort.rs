pub mod shell_sort {
  /// 希尔排序（Shell Sort）是一种插入排序的改进算法，它通过将待排序的序列分成多个子序列来进行排序，
  /// 然后逐步减小子序列的间隔，直到整个序列变为有序。希尔排序的关键在于选择合适的间隔序列。
  ///
  /// 希尔排序的时间复杂度取决于间隔序列的选择，但最坏情况下为 `O(n^2)`，平均情况下要好于常规的插入排序。
  /// 希尔排序不稳定，即相等元素的相对位置可能会发生改变。
  ///
  /// Shell Sort is an improvement over the insertion sort algorithm. It sorts the sequence by dividing it into
  /// multiple sub-sequences and then gradually reducing the gap between elements, until the entire sequence becomes sorted.
  /// The key to Shell Sort lies in selecting an appropriate gap sequence.
  ///
  /// The time complexity of Shell Sort depends on the gap sequence chosen, but it's worst-case time complexity is `O(n^2)`,
  /// which is better on average than the basic insertion sort. Shell Sort is not stable, meaning that the relative
  /// order of equal elements may change.
  ///
  /// # 参数 (Arguments)
  ///
  /// * `values`: 待排序的可变向量的引用。 (A mutable reference to the vector to be sorted.)
  ///
  /// # 示例 (Examples)
  ///
  /// ```
  /// let mut numbers = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
  /// shell_sort(&mut numbers);
  /// assert_eq!(numbers, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
  /// ```
  pub fn shell_sort<T: Ord + Copy>(values: &mut Vec<T>) {
    // 插入排序的变种，通过交换指定间隔的值并减小间隔至 1 来进行排序
    fn insertion<T: Ord + Copy>(values: &mut Vec<T>, start: usize, gap: usize) {
      for i in ((start + gap)..values.len()).step_by(gap) {
        let val_current = values[i]; // 当前需要插入的元素
        let mut pos = i; // 当前元素的位置

        // 向前比较并交换，使元素归位
        while pos >= gap && values[pos - gap] > val_current {
          values[pos] = values[pos - gap];
          pos -= gap;
        }

        values[pos] = val_current; // 将元素插入到正确的位置
      }
    }

    let mut count_sublist = values.len() / 2; // 将间隔初始化为数组长度的一半

    while count_sublist > 0 {
      for pos_start in 0..count_sublist {
        insertion(values, pos_start, count_sublist); // 对每个子序列进行插入排序
      }

      count_sublist /= 2; // 将间隔减半，继续排序
    }
  }
}

#[cfg(test)]
mod test {
  use crate::shell_sort::shell_sort;

  #[test]
  fn basic() {
    let mut vec = vec![3, 5, 6, 3, 1, 4];

    shell_sort(&mut vec);

    for i in 0..vec.len() - 1 {
      assert!(vec[i] <= vec[i + 1]);
    }
  }

  #[test]
  fn empty() {
    let mut vec: Vec<i32> = vec![];

    shell_sort(&mut vec);

    assert_eq!(vec, vec![]);
  }

  #[test]
  fn reverse() {
    let mut vec = vec![6, 5, 4, 3, 2, 1];

    shell_sort(&mut vec);

    for i in 0..vec.len() - 1 {
      assert!(vec[i] <= vec[i + 1]);
    }
  }

  #[test]
  fn already_sorted() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];

    shell_sort(&mut vec);

    for i in 0..vec.len() - 1 {
      assert!(vec[i] <= vec[i + 1]);
    }
  }
}

fn main() {}
