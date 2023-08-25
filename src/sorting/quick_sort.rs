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

/// 将可变切片围绕基准元素进行分区，确保左侧元素较小，右侧元素较大。
///
/// Partitions a mutable slice around a pivot element, ensuring elements on the left are smaller and elements on the right are larger.
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
fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
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
    use super::*;

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
