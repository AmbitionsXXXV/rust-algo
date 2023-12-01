// 时间复杂度 O(n + k), n: 输入数组的长度，k: 桶的数量
// 空间复杂度 O(n + k), n: 输入数组的长度，k: 桶的数量
pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
  // 判断输入数组 arr 是否为空，如果为空，则直接返回一个空的向量 vec![]，表示已经是有序状态
  if arr.is_empty() {
    return vec![];
  }

  // 获取数组中的最大值 max，以及数组长度 len
  let max = *arr.iter().max().unwrap();
  let len = arr.len();

  // 创建一个长度为 len + 1 的切片 buckets，用于存放桶，每个桶初始化为空向量
  let mut buckets = vec![vec![]; len + 1];

  // 遍历输入数组 arr 中的每个元素 i
  // 计算元素 i 应该放入的桶的索引位置，使用公式 len * i / max
  // 将元素 i 加入对应桶的末尾
  for i in arr {
    buckets[len * *i / max].push(*i);
  }

  // 遍历每个桶 bucket，对其中的元素进行排序，这里使用了内置的排序方法 sort_by
  for bucket in buckets.iter_mut() {
    // insertion_sort::insertion_sort(bucket);
    // 使用任何排序方法都可以，这里使用内置的排序方法
    bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
  }

  // 创建一个新的向量 result，用于存放最终排序结果
  let mut result = vec![];
  // 遍历每个桶 bucket，将桶中的元素按顺序加入到 result 中
  for bucket in buckets {
    for x in bucket {
      result.push(x);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::bucket_sort;
  use crate::sorting::counting_sort::is_sorted;

  #[test]
  fn empty() {
    let arr: [usize; 0] = [];
    let res = bucket_sort(&arr);
    assert!(is_sorted(&res));
  }

  #[test]
  fn one_element() {
    let arr: [usize; 1] = [4];
    let res = bucket_sort(&arr);
    assert!(is_sorted(&res));
  }

  #[test]
  fn already_sorted() {
    let arr: [usize; 3] = [10, 9, 105];
    let res = bucket_sort(&arr);
    assert!(is_sorted(&res));
  }

  #[test]
  fn basic() {
    let arr: [usize; 4] = [35, 53, 1, 0];
    let res = bucket_sort(&arr);
    assert!(is_sorted(&res));
  }

  #[test]
  fn odd_number_of_elements() {
    let arr: Vec<usize> = vec![1, 21, 5, 11, 58];
    let res = bucket_sort(&arr);
    assert!(is_sorted(&res));
  }

  #[test]
  fn repeated_elements() {
    let arr: Vec<usize> = vec![542, 542, 542, 542];
    let res = bucket_sort(&arr);
    assert!(is_sorted(&res));
  }
}

pub fn main() {}
