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
    use super::*;
    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }
    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
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

pub fn main(){
    let mut nums = [ 4, 1, 3, 1, 5, 2 ];
    bubble_sort(&mut nums);
    println!("{:?}", nums);
}