pub fn main() {}

/// 堆排序（Heap Sort）是一种高效的排序算法，它利用二叉堆这种数据结构进行排序。堆排序分为两个主要步骤：建堆和排序
/// 第一步:建堆. 从数组的中间位置开始，逐个将子树调整为最大堆
/// 第二步:排序. 将堆顶（最大元素）与数组的最后一个元素交换，再将剩余的子数组重新调整为最大堆.重复这个过程直到排序完成
///
/// # Arguments
///
/// * `arr`:
///
/// returns: ()
pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let size = arr.len();

    // 构建最大堆
    for i in (0..size / 2).rev() {
        heapify(arr, i, size);
    }

    // 每轮循环将堆顶元素（也就是最大元素）放到最后
    for i in (1..size).rev() {
        arr.swap(0, i);
        // 恢复最大堆
        heapify(arr, 0, i);
    }
}

fn heapify<T: PartialOrd>(arr: &mut [T], root: usize, end: usize) {
    // 记录父节点和左右节点中最大元素的索引位置
    let mut largest = root;
    let left_child = 2 * root + 1;

    if left_child < end && arr[left_child] > arr[largest] {
        largest = left_child;
    }

    let right_child = left_child + 1;

    if right_child < end && arr[right_child] > arr[largest] {
        largest = right_child;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, largest, end);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        heap_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        heap_sort(&mut vec);
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
        heap_sort(&mut vec);
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
