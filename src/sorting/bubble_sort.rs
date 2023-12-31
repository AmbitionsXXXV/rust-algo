pub fn main() {
  let mut arr = [4, 1, 3, 1, 5, 2];
  bubble_sort(&mut arr);
  println!("{:?}", arr);
}

/* 冒泡排序 */
pub fn bubble_sort_without_flag<T: PartialOrd>(arr: &mut [T]) {
  // 外循环：未排序区间为 [0, i]
  for i in (1..arr.len()).rev() {
    // 内循环：将未排序区间 [0, i] 中的最大元素交换至该区间的最右端
    for j in 0..i {
      if arr[j] > arr[j + 1] {
        // 交换 arr[j] 与 arr[j + 1]
        arr.swap(j, j + 1);
      }
    }
  }
}

// PartialOrd 用于实现对于可比较类型的值进行有序比较
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  if arr.len() <= 1 {
    return;
  }

  let size = arr.len();

  for i in 0..(size - 1) {
    // 记录是否有交换发生,有助于特殊情况下降低时间复杂度
    let mut swapped = false;

    for j in 1..(size - i) {
      if arr[j - 1] > arr[j] {
        arr.swap(j - 1, j);
        swapped = true;
      }
    }

    // 如果没有交换发生，说明已经排好序了
    if !swapped {
      break;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{bubble_sort, bubble_sort_without_flag};

  #[test]
  fn test_empty_vec() {
    let mut empty_vec: Vec<String> = vec![];
    bubble_sort(&mut empty_vec);
    assert_eq!(empty_vec, Vec::<String>::new());
  }

  #[test]
  fn test_empty_vec_without_flag() {
    let mut empty_vec: Vec<String> = vec![];
    bubble_sort_without_flag(&mut empty_vec);
    assert_eq!(empty_vec, Vec::<String>::new());
  }

  #[test]
  fn test_number_vec() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    bubble_sort(&mut vec);
    assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
  }

  #[test]
  fn test_number_vec_without_flag() {
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    bubble_sort_without_flag(&mut vec);
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
    bubble_sort(&mut vec);
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
