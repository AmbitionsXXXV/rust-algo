pub fn main() {}

/// Reverses a given string.
///
/// This function takes an input string and returns a new string with the characters reversed.
/// It utilizes the `chars()` method to obtain an iterator over the characters in the input string,
/// and then uses the `rev()` method to reverse the order of these characters.
/// Finally, the reversed characters are collected into a new string.
///
/// # Arguments
///
/// * `text` - The input string to be reversed.
///
/// # Returns
///
/// A new `String` containing the reversed characters of the input string.
///
/// # Examples
///
/// ```
/// let reversed = reverse("hello");
/// assert_eq!(reversed, "olleh");
/// ```
///
/// # Complexity
///
/// The time complexity of this function is O(n), where n is the length of the input string.
/// This is because iterating over the characters, reversing them, and collecting them into
/// a new string all take linear time with respect to the length of the string.
///
/// The space complexity is also O(n), as the new string holding the reversed characters
/// requires additional memory proportional to the length of the input string.
///
/// ---------
///
/// 反转给定的字符串。
///
/// 该函数接受一个输入字符串，并返回一个新字符串，其中包含反转后的字符。
/// 它使用`chars()`方法获取输入字符串的字符迭代器，然后使用`rev()`方法来反转这些字符的顺序。
/// 最后，将反转后的字符收集到一个新的字符串中。
///
/// # 参数
///
/// * `text` - 要进行反转的输入字符串。
///
/// # 返回值
///
/// 包含输入字符串反转字符的新的`String`。
///
/// # 示例
///
/// ```
/// let reversed = reverse("hello");
/// assert_eq!(reversed, "olleh");
/// ```
///
/// # 复杂度
///
/// 该函数的时间复杂度为O(n)，其中n是输入字符串的长度。
/// 这是因为遍历字符、反转字符顺序以及收集字符到新的字符串中，都需要与输入字符串长度成线性关系的时间。
///
/// 空间复杂度也是O(n)，因为保存反转后的字符所创建的新字符串需要的额外内存与输入字符串的长度成比例。
pub fn reverse(text: &str) -> String {
    // 获取输入字符串的字符迭代器
    // Obtain an iterator over the characters of the input string
    let char_iterator = text.chars();

    // 将反转后的字符收集到一个新的字符串中
    // Reverse the order of characters using the `rev()` method
    let reversed_chars = char_iterator.rev();

    // 将反转后的字符收集到一个新的字符串中
    // Collect the reversed characters into a new String
    let reversed_text = reversed_chars.collect();

    // 返回反转后的字符串
    // Return the reversed string
    reversed_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(reverse("racecar"), "racecar");
    }

    #[test]
    fn test_sentence() {
        assert_eq!(reverse("step on no pets"), "step on no pets");
    }
}
