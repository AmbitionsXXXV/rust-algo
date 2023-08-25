/// 臭皮匠排序（Stooge Sort）是一种简单而效率较低的排序算法，其基本思想是递归地对未排序部分进行排序，
/// 通过不断缩小排序范围来确保序列的有序性。
///
/// 臭皮匠排序的时间复杂度较高，为 O(n^(log 3 / log 1.5))。尽管其在实际应用中不常用，
/// 但在教学和理论讨论中可能被用作示例。
///
/// Stooge Sort is a simple yet inefficient sorting algorithm. It works by recursively sorting the unsorted portion
/// of the sequence, ensuring the order by gradually reducing the sorting range.
///
/// The time complexity of Stooge Sort is high, O(n^(log 3 / log 1.5)). While it's not commonly used in practice,
/// it may serve as an example in teaching and theoretical discussions.
///
/// Helper function: Performs Stooge Sort on a specified range of the slice.
///
/// # 参数 (Arguments)
///
/// * `arr`: 待排序的可变切片的引用。 (A mutable reference to the slice to be sorted.)
/// * `start`: The start index of the range to be sorted.
/// * `end`: The end index of the range to be sorted.
///
/// # 示例 (Examples)
///
/// ```rust
/// let mut numbers = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
/// stooge_sort(&mut numbers);
/// assert_eq!(numbers, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
/// ```
fn _stooge_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    // 如果起始元素比结束元素大，则交换它们，确保起始元素较小。
    // If the start element is greater than the end element, swap them to ensure the start element is smaller.
    if arr[start] > arr[end] {
        arr.swap(start, end);
    }

    // 如果范围中的元素不足三个，无需排序，直接返回。
    // If there are fewer than three elements in the range, no need to sort, just return.
    if start + 1 >= end {
        return;
    }

    // 计算间隔，将序列分成三个部分。
    // Calculate the interval to divide the sequence into three parts.
    let k = (end - start + 1) / 3;

    // 分别对前两个部分和后两个部分递归进行排序。
    // Recursively sort the first two parts and the last two parts separately.
    _stooge_sort(arr, start, end - k);
    _stooge_sort(arr, start + k, end);
    _stooge_sort(arr, start, end - k);
}

/// 对切片进行臭皮匠排序。
///
/// Performs Stooge Sort on a slice.
///
/// # 参数 (Arguments)
///
/// * `arr`: 待排序的可变切片的引用。 (A mutable reference to the slice to be sorted.)
///
/// # 示例 (Examples)
///
/// ```
/// let mut numbers = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
/// stooge_sort(&mut numbers);
/// assert_eq!(numbers, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
/// ```
pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    // 如果切片为空，直接返回。
    // If the slice is empty, just return.
    if len == 0 {
        return;
    }

    // 调用辅助函数，对整个切片进行排序。
    // Call the helper function to sort the entire slice.
    _stooge_sort(arr, 0, len - 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];

        stooge_sort(&mut vec);

        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];

        stooge_sort(&mut vec);

        assert_eq!(vec, vec![]);
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];

        stooge_sort(&mut vec);

        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];

        stooge_sort(&mut vec);

        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}
