pub fn main() {}

pub mod comb_sort {
    /// 梳排序（Comb Sort）是一种简单的排序算法，它是冒泡排序的改进版。梳排序通过比较和交换相隔一定间隔的元素来消除小的波动，从而提高了冒泡排序的效率。
    ///
    /// 梳排序的基本思想是：通过比较两个元素的大小并交换它们的位置，将较大的元素“梳过”较小的元素。该过程会逐渐减小比较的间隔，
    /// 直到间隔为1，即回到普通的冒泡排序。通过这种方式，梳排序能够在开始时消除较大间隔带来的逆序对，从而提高整体的排序效率。
    ///
    /// 下面是梳排序的基本实现步骤：
    ///
    /// 1. 初始化间隔 gap，可以是数组长度的一个固定系数（例如1.3），也可以是自定义的初始值。
    /// 2. 通过比较相隔 gap 的元素，将较大的元素往后移动。
    /// 3. 重复上述步骤，缩小 gap 的值，直到最后一个 gap 为1，执行一次普通的冒泡排序。
    /// 4. 当 gap 为1时，排序完成
    pub fn comb_sort<T: Ord>(arr: &mut [T]) {
        let mut gap = arr.len();
        let shrink = 1.3;
        let mut sorted = false;

        while !sorted {
            gap = (gap as f32 / shrink).floor() as usize;

            if gap <= 1 {
                gap = 1;
                sorted = true;
            }

            for i in 0..arr.len() - gap {
                let j = i + gap;

                if arr[i] > arr[j] {
                    arr.swap(i, j);
                    sorted = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb_sort::comb_sort;

    #[test]
    fn descending() {
        // descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];

        comb_sort(&mut ve1);

        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        // pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];

        comb_sort(&mut ve2);

        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
