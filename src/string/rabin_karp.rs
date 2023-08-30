/// Searches for occurrences of the `pattern` string within the `target` string using the Rabin-Karp algorithm.
///
/// The Rabin-Karp algorithm is a string searching algorithm that uses hashing to quickly locate the occurrences
/// of a pattern within a longer text.
///
/// # Arguments
///
/// * `target` - The target string in which to search for occurrences of the `pattern`.
/// * `pattern` - The pattern string that needs to be found within the `target`.
///
/// # Returns
///
/// A vector containing the starting indices of all occurrences of the `pattern` within the `target`.
///
/// 在目标字符串中使用 Rabin-Karp 算法搜索模式字符串的出现位置。
///
/// Rabin-Karp 算法是一种字符串搜索算法，它使用哈希函数来快速定位较长文本中模式字符串的出现位置。
///
/// # 参数
///
/// * `target` - 目标字符串，用于搜索其中的模式字符串出现位置。
/// * `pattern` - 需要在目标字符串中查找的模式字符串。
///
/// # 返回值
///
/// 包含所有模式字符串出现位置的起始索引的向量。
pub fn rabin_karp(target: String, pattern: String) -> Vec<usize> {
    // Quick exit
    if target.is_empty() || pattern.is_empty() || pattern.len() > target.len() {
        return vec![];
    }

    // Convert pattern to a string to avoid mutability issues
    // 将模式字符串转换为新的字符串，以避免可变性问题
    let string: String = (&pattern[0..pattern.len()]).to_string();

    // Calculate hash of the pattern
    // 计算模式字符串的哈希值
    let hash_pattern = hash(string.clone());
    let mut ret = vec![];

    // Iterate through the target string
    // 遍历目标字符串
    for i in 0..(target.len() - pattern.len() + 1) {
        // Extract a substring of the same length as the pattern
        // 提取与模式字符串长度相同的子字符串
        let s = (&target[i..(i + pattern.len())]).to_string();

        // Calculate hash of the current substring
        // 计算当前子字符串的哈希值
        let string_hash = hash(s.clone());

        // Compare hashes and full strings to find matches
        // 比较哈希值和完整字符串以找到匹配项
        if string_hash == hash_pattern && s == string {
            // Store the starting index of the match
            // 存储匹配项的起始索引
            ret.push(i);
        }
    }

    ret
}

/// Calculates the hash value of a string using the Rabin-Karp hash function.
///
/// The Rabin-Karp hash function is used to generate a hash value for a string based on its ASCII values and a prime number.
///
/// # Arguments
///
/// * `s` - The input string for which to calculate the hash value.
///
/// # Returns
///
/// The calculated hash value of the input string.
///
/// 使用 Rabin-Karp 哈希函数计算字符串的哈希值。
///
/// Rabin-Karp 哈希函数基于字符串的 ASCII 值和一个质数来生成哈希值。
///
/// # 参数
///
/// * `s` - 需要计算哈希值的输入字符串。
///
/// # 返回值
///
/// 输入字符串的计算哈希值。
fn hash(mut s: String) -> u16 {
    let prime: u16 = 101;
    let last_char = s
        .drain(s.len() - 1..)
        .next()
        .expect("Failed to get the last char of the string");
    let mut res: u16 = 0;

    // Calculate hash using ASCII values and a rolling hash approach
    // 使用 ASCII 值和滚动哈希的方法计算哈希值
    // 1. 我们首先获取字符串的第一个字符的 ASCII 值，将其乘以 256，然后对一个质数（在这里是 101）取模，得到初始的哈希值 res。
    // 2. 随后，我们遍历字符串中的每个字符（除了第一个字符），将其 ASCII 值加到 res 中，并对质数取模。这相当于将当前字符的影响叠加到先前的哈希值中。
    // 3. 由于我们将当前字符的 ASCII 值添加到 res 中，我们需要确保 res 不会溢出，因此我们再次对质数取模。
    // 这样，我们通过对每个字符的 ASCII 值应用一系列操作，就能够计算出整个字符串的哈希值。在滑动窗口移动时，我们只需添加新字符的影响并去除旧字符的影响，从而快速更新哈希值
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if i == 0 {
            res = (c as u16 * 256) % prime;
        } else {
            res = (((res + c as u16) % 101) * 256) % 101;
        }
    }

    (res + last_char as u16) % prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hi_hash() {
        let hash_result = hash("hi".to_string());

        assert_eq!(hash_result, 65);
    }

    #[test]
    fn abr_hash() {
        let hash_result = hash("abr".to_string());

        assert_eq!(hash_result, 4);
    }

    #[test]
    fn bra_hash() {
        let hash_result = hash("bra".to_string());

        assert_eq!(hash_result, 30);
    }

    // Attribution to @pgimalac for his tests from Knuth-Morris-Pratt
    #[test]
    fn each_letter_matches() {
        let index = rabin_karp("aaa".to_string(), "a".to_string());

        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn a_few_separate_matches() {
        let index = rabin_karp("abababa".to_string(), "ab".to_string());

        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn one_match() {
        let index = rabin_karp("ABC ABCDAB ABCDABCDABDE".to_string(), "ABCDABD".to_string());

        assert_eq!(index, vec![15]);
    }

    #[test]
    fn lots_of_matches() {
        let index = rabin_karp("aaabaabaaaaa".to_string(), "aa".to_string());

        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn lots_of_intricate_matches() {
        let index = rabin_karp("ababababa".to_string(), "aba".to_string());

        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn not_found0() {
        let index = rabin_karp("abcde".to_string(), "f".to_string());

        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found1() {
        let index = rabin_karp("abcde".to_string(), "ac".to_string());

        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found2() {
        let index = rabin_karp("ababab".to_string(), "bababa".to_string());

        assert_eq!(index, vec![]);
    }

    #[test]
    fn empty_string() {
        let index = rabin_karp("".to_string(), "abcdef".to_string());

        assert_eq!(index, vec![]);
    }
}
