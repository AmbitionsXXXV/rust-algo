/// 实现了Knuth-Morris-Pratt（KMP）算法，用于在文本中查找子字符串的出现位置。
/// Implements the Knuth-Morris-Pratt (KMP) algorithm for substring search.
///
/// Knuth-Morris-Pratt算法通过利用部分匹配表（partial match table）高效地在给定的文本字符串中搜索模式字符串的出现位置。
/// The Knuth-Morris-Pratt algorithm efficiently searches for occurrences of a pattern string
/// within a given text string by utilizing a partial match table.
///
/// # 参数 (Arguments)
///
/// * `st` - 要在其中搜索模式的文本字符串。
///          The text string in which to search for the pattern.
/// * `pat` - 要在文本中搜索的模式字符串。
///          The pattern string to search for within the text.
///
/// # 返回值 (Returns)
///
/// A vector containing the starting indices of all occurrences of the pattern within the text.
/// 包含所有模式出现位置起始索引的向量。
///
/// # 示例 (Examples)
///
/// ```
/// let text = "ABABDABACDABABCABAB";
/// let pattern = "ABABCABAB";
/// let indices = knuth_morris_pratt(text.to_owned(), pattern.to_owned());
/// assert_eq!(indices, vec![10]);
/// ```
///
/// # 算法概述 (Algorithm Overview)
///
/// The KMP algorithm constructs a partial match table that helps determine the maximum length of proper
/// prefix that is also a proper suffix for each substring of the pattern.
/// This table is then used to efficiently navigate through the text while searching for occurrences of the pattern.
/// By avoiding unnecessary character comparisons, the algorithm achieves linear time complexity.
/// KMP算法构建了一个部分匹配表，用于确定模式的每个子字符串的最大前缀，该前缀也是相应子字符串的最大后缀。
/// 然后，该表被用于高效地遍历文本，以寻找模式的出现位置。通过避免不必要的字符比较，该算法实现了线性时间复杂度。
///
/// # 复杂度 (Complexity)
///
/// The time complexity of this function is O(n + m), where n is the length of the text and m is the length of the pattern.
/// This is because both the building of the partial match table and the substring search itself have linear time complexity.
/// The space complexity is O(m), due to the space used for the partial match table.
/// 该函数的时间复杂度为O(n + m)，其中n是文本的长度，m是模式的长度。
/// 这是因为构建部分匹配表和子字符串搜索本身都具有线性时间复杂度。
/// 空间复杂度为O(m)，用于存储部分匹配表的空间。
///
/// # 实现细节 (Implementation Details)
///
/// The function first converts the input strings into byte arrays for efficient indexing.
/// It then constructs the partial match table using a loop and builds the vector of starting indices of occurrences.
/// The KMP algorithm uses two pointers, `i` and `j`, to navigate through the text and pattern respectively.
/// The algorithm iterates through the text, adjusting the `j` pointer using the partial match table,
/// and updating the `ret` vector whenever a full pattern match is found.
/// 函数首先将输入字符串转换为字节数组，以实现高效索引。
/// 接着，它使用循环构建部分匹配表，并构建包含出现位置起始索引的向量。
/// KMP算法使用两个指针，`i` 和 `j`，分别在文本和模式中导航。
/// 算法遍历文本，使用部分匹配表调整`j`指针，并在找到完整模式匹配时更新`ret`向量。
pub fn knuth_morris_pratt(st: String, pat: String) -> Vec<usize> {
    // 如果文本或模式为空，则返回一个空向量
    // Return an empty vector if either the text or pattern is empty
    if st.is_empty() || pat.is_empty() {
        return vec![];
    }

    // 将输入字符串转换为字节数组以实现高效索引
    // Convert the input strings to byte arrays for efficient indexing
    let string = st.into_bytes();
    let pattern = pat.into_bytes();

    // 使用第一个元素初始化部分匹配表
    // Initialize the partial match table with the first element
    let mut partial = vec![0];

    // 使用KMP算法构建部分匹配表
    // Build the partial match table using the KMP algorithm
    for i in 1..pattern.len() {
        // 使用前一个值初始化模式指针j
        // Initialize the pattern pointer j with the previous value from the partial match table
        let mut j = partial[i - 1];

        // 使用部分匹配表调整模式指针j，直到找到合适的位置
        // Use the partial match table to adjust the pattern pointer j, finding a proper position
        while j > 0 && pattern[j] != pattern[i] {
            j = partial[j - 1];
        }

        // 计算下一个部分匹配表的值并存储
        // Calculate the next value for the partial match table and store it
        partial.push(if pattern[j] == pattern[i] { j + 1 } else { j });
    }

    // 初始化向量以存储出现位置起始索引
    // Initialize the vector to store starting indices of occurrences
    let mut ret = vec![];
    // 将模式指针j初始化为0
    // Initialize the pattern pointer j to 0
    let mut j = 0;

    // 遍历文本以查找模式的出现位置
    // Iterate through the text to find pattern occurrences
    for (i, &c) in string.iter().enumerate() {
        // 使用部分匹配表调整模式指针j，直到找到合适的位置
        // Use the partial match table to adjust the pattern pointer j, finding a proper position
        while j > 0 && c != pattern[j] {
            j = partial[j - 1];
        }

        // 如果字符匹配，增加模式指针j
        // If the characters match, increment the pattern pointer j
        if c == pattern[j] {
            j += 1;
        }

        // 如果找到完整模式匹配，更新结果向量
        // If a full pattern match is found, update the result vector
        if j == pattern.len() {
            ret.push(i + 1 - j);
            // 使用部分匹配表更新模式指针j
            // Update the pattern pointer j using the partial match table
            j = partial[j - 1];
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_letter_matches() {
        let index = knuth_morris_pratt("aaa".to_string(), "a".to_string());

        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn a_few_separate_matches() {
        let index = knuth_morris_pratt("abababa".to_string(), "ab".to_string());

        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn one_match() {
        let index =
            knuth_morris_pratt("ABC ABCDAB ABCDABCDABDE".to_string(), "ABCDABD".to_string());

        assert_eq!(index, vec![15]);
    }

    #[test]
    fn lots_of_matches() {
        let index = knuth_morris_pratt("aaabaabaaaaa".to_string(), "aa".to_string());

        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn lots_of_intricate_matches() {
        let index = knuth_morris_pratt("ababababa".to_string(), "aba".to_string());

        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn not_found0() {
        let index = knuth_morris_pratt("abcde".to_string(), "f".to_string());

        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found1() {
        let index = knuth_morris_pratt("abcde".to_string(), "ac".to_string());

        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found2() {
        let index = knuth_morris_pratt("ababab".to_string(), "bababa".to_string());

        assert_eq!(index, vec![]);
    }

    #[test]
    fn empty_string() {
        let index = knuth_morris_pratt("".to_string(), "abcdef".to_string());

        assert_eq!(index, vec![]);
    }
}
