fn main() {}

pub mod linear_search {
    use std::cmp::PartialEq;

    /// 这个函数用于在给定数组中进行线性搜索，查找特定元素的位置。
    ///
    /// This function performs a linear search in the given array to find the position of a specific element.
    ///
    /// # 参数 (Arguments)
    /// * `item` - 要查找的元素的引用 (A reference to the element to search for)。
    /// * `arr` - 输入的数组 (The input array)。
    ///
    /// # 返回值 (Returns)
    /// 返回找到的元素的索引，如果未找到则返回None。
    ///
    /// Returns the index of the found element, or None if not found
    ///
    /// # 示例 (Examples)
    /// ```
    /// let array = vec![1, 2, 3, 4, 5];
    /// let target = 3;
    /// let result = linear_search(&target, &array);
    /// assert_eq!(result, Some(2));
    /// ```
    ///
    /// # 注意事项 (Note)
    /// 该函数要求元素类型T实现了PartialEq trait以进行比较
    ///
    /// This function requires that the element type T implements the PartialEq trait for comparison
    pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
        for (i, data) in arr.iter().enumerate() {
            if item == data {
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::linear_search::linear_search;

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = linear_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));
        let index = linear_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));
        let index = linear_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));
        let index = linear_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn empty() {
        let index = linear_search(&1, &vec![]);
        assert_eq!(index, None);
    }
}
