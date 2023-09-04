use crate::cocktail_shaker_sort::cocktail_shaker_sort;

pub fn main() {
  let mut arr = [4, 1, 3, 1, 5, 2];
  cocktail_shaker_sort(&mut arr);
  println!("{:?}", arr);
}

pub mod cocktail_shaker_sort {
  /// 鸡尾酒排序Cocktail Sort(双向冒泡排序Bidirectional Bubble Sort)
  /// 冒泡排序不同的地方在于，鸡尾酒排序通过从左到右和从右到左交
  /// 具体实现步骤如下：
  ///
  /// 1.初始化边界变量 start 和 end，分别指向待排序序列的起始和结束位置；
  /// 2.创建一个布尔型变量 swapped，用于标记本轮迭代是否有元素交换发生；
  /// 3.进入循环，直到一轮迭代没有任何交换发生为止：
  ///   · 从 start 到 end-1，对当前位置和下一个位置的元素进行比较，如果前一个元素大于后一个元素，则交换它们，并将 swapped 设置为 true；
  ///   · 如果 swapped 仍然为 false，表示本轮迭代没有元素交换，排序完成，退出循环；
  ///   · 将 swapped 重置为 false；
  ///   · 将 end 减小 1，表示已经将最大的元素放置到了正确的位置上；
  ///   · 从 end 到 start+1，对当前位置和前一个位置的元素进行比较，如果前一个元素大于后一个元素，则交换它们，并将 swapped 设置为 true；
  ///   · 将 swapped 重置为 false；
  ///   · 将 start 增加 1，表示已经将最小的元素放置到了正确的位置上；替地进行元素比较和交换，而不仅仅是从左到右
  /// 鸡尾酒排序在某些情况下可以减少排序的回合数，特别是对于一些部分有序的数组，因为它可以从两个方向同时进行元素交换。
  /// 然而，需要注意的是，鸡尾酒排序的最坏时间复杂度仍然是O(n^2)，与冒泡排序相同。因此，在实际应用中，对于大数据集合，更常见的排序算法（如快速排序或归并排序）可能更具效率。
  pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len == 0 {
      return;
    }

    loop {
      let mut swapped = false;

      // clamp() 方法用于将一个值限制在指定的范围内
      for i in 0..(len - 1).clamp(0, len) {
        if arr[i] > arr[i + 1] {
          arr.swap(i, i + 1);
          swapped = true;
        }
      }

      if !swapped {
        break;
      }

      swapped = false;

      for i in (0..(len - 1)).rev() {
        if arr[i] > arr[i + 1] {
          arr.swap(i, i + 1);
          swapped = true;
        }
      }

      if !swapped {
        break;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let mut arr = vec![5, 2, 1, 3, 4, 6];
    cocktail_shaker_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
  }

  #[test]
  fn empty() {
    let mut arr = Vec::<i32>::new();
    cocktail_shaker_sort(&mut arr);
    assert_eq!(arr, vec![]);
  }

  #[test]
  fn one_element() {
    let mut arr = vec![1];
    cocktail_shaker_sort(&mut arr);
    assert_eq!(arr, vec![1]);
  }

  #[test]
  fn pre_sorted() {
    let mut arr = vec![1, 2, 3, 4, 5, 6];
    cocktail_shaker_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
  }
}
