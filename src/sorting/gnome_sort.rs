pub mod gnome_sort {
    /// 地精排序（Gnome Sort），也称为Stupid Sort或Bogo Sort，是一种简单但非常低效的排序算法。它通过不断比较相邻的元素并交换它们，直到达到正确的顺序为止
    ///
    /// # Arguments
    ///
    /// * `arr`:
    ///
    /// returns: Vec<T, Global>
    pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        let mut arr = arr.to_vec();
        let mut i: usize = 1;
        let mut j: usize = 2;

        while i < arr.len() {
            if arr[i - 1] < arr[i] {
                i = j;
                j = i + 1;
            } else {
                arr.swap(i - 1, i);
                i -= 1;

                if i == 0 {
                    i = j;
                    j += 1;
                }
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::gnome_sort::gnome_sort;

    #[test]
    fn basic() {
        let res = gnome_sort(&vec![6, 5, -8, 3, 2, 3]);
        assert_eq!(res, vec![-8, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn already_sorted() {
        let res = gnome_sort(&vec!["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = gnome_sort(&vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let res = gnome_sort(&vec![3]);
        assert_eq!(res, vec![3]);
    }

    #[test]
    fn empty() {
        let res = gnome_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_gnome_sort_empty_string() {
        let res = gnome_sort(&vec!["", "c", "", "a", "b", ""]);
        assert_eq!(res, vec!["", "", "", "a", "b", "c"]);
    }
}

fn main() {}
