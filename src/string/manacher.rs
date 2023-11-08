/// The Manacher algorithm is an efficient linear-time algorithm used to find the longest palindromic substring within a given string.
/// It utilizes the concept of palindrome symmetry to optimize the process.
///
/// Manacher算法是一种线性时间复杂度的算法，用于在给定字符串中找到最长回文子串。
/// 它利用回文对称性的概念来优化处理过程。
///
/// # Arguments
///
/// * `s` - The input string in which the longest palindromic substring needs to be found.
///
/// # 参数
///
/// * `s` - 需要找到最长回文子串的输入字符串。
///
/// # Returns
///
/// The longest palindromic substring found.
///
/// # 返回值
///
/// 找到的最长回文子串。
///
/// # Example
///
/// ```
/// let input = String::from("babad");
/// let longest_palindrome = manacher(input);
/// assert_eq!(longest_palindrome, "bab");
/// ```
pub fn manacher(s: String) -> String {
  let l = s.len();

  if l <= 1 {
    return s;
  }

  // MEMO: We need to detect odd palindrome as well, therefore, inserting dummy string so that
  // we can find a pair with dummy center character.
  // MEMO: 我们需要检测奇数长度的回文，因此插入虚拟字符以便
  // 我们可以找到具有虚拟中心字符的一对。
  // Create a vector `chars` with a pre-allocated capacity for efficient memory management.
  // The capacity is calculated based on the original string length multiplied by 2, plus 1 for additional placeholders.
  // 使用预分配的容量创建向量 `chars`，以实现有效的内存管理。
  // 容量的计算基于原始字符串长度乘以 2，再加上 1 用于额外的占位符。
  let mut chars: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);

  for c in s.chars() {
    chars.push('#');
    chars.push(c);
  }

  chars.push('#');

  // List: storing the length of palindrome at each index of string
  // 列表：存储字符串每个索引处的回文长度
  let mut length_of_palindrome = vec![1usize; chars.len()];
  // Integer: Current checking palindrome's center index
  // 整数：当前检查回文的中心索引
  let mut current_center: usize = 0;
  // Integer: Right edge index existing the radius away from current center
  // 整数：距离当前中心的右边缘索引
  let mut right_from_current_center: usize = 0;

  for i in 0..chars.len() {
    // 1: Check if we are looking at the right side of palindrome.
    // 1：检查是否在回文的右侧。
    if right_from_current_center > i && i > current_center {
      // 1-1: If so, copy from the left side of palindrome.
      // If the value + index exceeds the right edge index, we should cut and check palindrome later #3.
      // 1-1：如果是，从回文的左侧复制。
      // 如果值 + 索引超过右边缘索引，我们应该在以后剪切并检查回文 #3。
      length_of_palindrome[i] = std::cmp::min(
        right_from_current_center - i,
        length_of_palindrome[2 * current_center - i],
      );

      // 1-2: Move the checking palindrome to a new index if it exceeds the right edge.
      // 1-2：如果检查的回文超过了右边缘。
      if length_of_palindrome[i] + i >= right_from_current_center {
        current_center = i;
        right_from_current_center = length_of_palindrome[i] + i;

        // 1-3: If the radius exceeds the end of the list, it means checking is over.
        // You will never get a larger value because the string will only get shorter.
        // 1-3：如果半径超过列表的末尾，表示检查结束。
        // 永远不会得到更大的值，因为字符串只会变短。
        if right_from_current_center >= chars.len() - 1 {
          break;
        }
      } else {
        // 1-4: If the checking index doesn't exceed the right edge,
        // it means the length is just the same as the left side.
        // You don't need to check anymore.
        // 1-4：如果检查索引未超过右边缘，
        // 那么长度与左侧相同。不需要再检查。
        continue;
      }
    }

    // Integer: Current radius from the checking index
    // If it's copied from the left side and more than 1,
    // it means it's ensured, so you don't need to check inside the radius.
    // 整数：当前半径从检查索引开始
    // 如果从左侧复制且大于1，则已确保不需要再检查半径内部。
    let mut radius: usize = (length_of_palindrome[i] - 1) / 2;
    radius += 1;
    // 2: Checking palindrome.
    // Need to care about overflow usize.
    // 2：检查回文。
    // 需要注意 usize 的溢出。
    while i >= radius && i + radius <= chars.len() - 1 && chars[i - radius] == chars[i + radius] {
      length_of_palindrome[i] += 2;
      radius += 1;
    }
  }

  // 3: Find the maximum length and generate the answer.
  // 3：寻找最大长度并生成答案。
  let center_of_max = length_of_palindrome
    .iter()
    .enumerate()
    .max_by_key(|(_, &value)| value)
    .map(|(idx, _)| idx)
    .unwrap();

  // Calculate the radius of the maximum palindrome centered at `center_of_max`.
  // 计算以 `center_of_max` 为中心的最大回文串的半径。
  let radius_of_max = (length_of_palindrome[center_of_max] - 1) / 2;

  // Generate the answer by extracting the substring from `chars` using the calculated indices.
  // 通过使用计算出的索引从 `chars` 中提取子字符串来生成答案。
  let answer = &chars[(center_of_max - radius_of_max)..(center_of_max + radius_of_max + 1)]
    .iter()
    .collect::<String>();

  // Remove the '#' characters from the answer to get the final result.
  // 从答案中移除 '#' 字符，以获得最终结果。
  answer.replace("#", "")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_longest_palindrome_by_manacher() {
    assert_eq!(manacher("babad".to_string()), "aba".to_string());
    assert_eq!(manacher("cbbd".to_string()), "bb".to_string());
    assert_eq!(manacher("a".to_string()), "a".to_string());

    let ac_ans = manacher("ac".to_string());

    assert!(ac_ans == "a".to_string() || ac_ans == "c".to_string());
  }
}

fn main() {}
