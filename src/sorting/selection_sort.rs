pub mod selection_sort {
    /// 选择排序（Selection Sort）是一种简单的排序算法，它的思想是在未排序部分中选择最小（或最大）的元素，
    /// 并将其与已排序部分的末尾进行交换，从而逐步构建有序序列。
    ///
    /// Selection Sort is a simple sorting algorithm that works by repeatedly selecting the minimum (or maximum) element from
    /// the unsorted part of the array and swapping it with the element at the end of the sorted portion, gradually building
    /// up the sorted sequence.
    ///
    /// 选择排序的时间复杂度始终为 `O(n^2)`，无论输入数据的分布情况如何。虽然时间复杂度较高，
    /// 但选择排序的优势在于它的简单性和不占用额外的内存空间。
    ///
    /// The time complexity of Selection Sort is always O(n^2), regardless of the distribution of input data. Although it has
    /// a higher time complexity, its advantage lies in its simplicity and minimal use of extra memory.
    ///
    /// # 参数 (Arguments)
    ///
    /// * `arr`: 待排序的可变切片的引用。 (A mutable reference to the slice to be sorted.)
    ///
    /// # 示例 (Examples)
    ///
    /// ```
    /// let mut numbers = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    /// selection_sort(&mut numbers);
    /// assert_eq!(numbers, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    /// ```
    pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
        // 如果数组长度小于等于 1，则无需排序 (If the length of the array is less than or equal to 1, no sorting is needed)
        if arr.len() <= 1 {
            return;
        }

        let size = arr.len();

        // 通过遍历数组，每次选择最小元素并将其与当前位置进行交换 (Iterate through the array, selecting the minimum element each time and swapping it with the current position)
        for i in 0..(size - 1) {
            // 找到未排序部分的最小元素的索引 (Find the index of the minimum element in the unsorted portion)
            let mut min_index = i;

            // 遍历未排序部分，找到最小元素的索引 (Iterate through the unsorted portion to find the index of the minimum element)
            for j in (i + 1)..size {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }

            // 如果最小元素不在当前位置，进行交换 (If the minimum element is not at the current position, swap it)
            if min_index != i {
                arr.swap(i, min_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::selection_sort::selection_sort;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        selection_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        selection_sort(&mut vec);
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
        selection_sort(&mut vec);
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

fn main() {
    let mut arr = [7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    selection_sort(&mut arr);
    println!("{:?}", arr);
}
