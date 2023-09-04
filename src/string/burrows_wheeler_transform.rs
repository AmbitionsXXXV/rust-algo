/// Applies the Burrows-Wheeler Transform (BWT) to a given input string.
/// 对给定的输入字符串应用 Burrows-Wheeler 变换（BWT）。
///
/// The Burrows-Wheeler Transform is a reversible text transformation that rearranges the characters
/// of the input string in a way that makes it more amenable to compression.
/// Burrows-Wheeler 变换是一种可逆的文本变换，通过重新排列输入字符串的字符，使其更易于压缩。
///
/// This function takes an input string and returns a tuple containing the transformed string and the index of the
/// original string within the sorted rotations.
/// 此函数接受一个输入字符串，并返回一个元组，其中包含变换后的字符串和原始字符串在排序旋转表中的索引。
///
/// # Arguments
/// # 参数
///
/// * `input` - The input string to which the BWT will be applied.
/// * `input` - 将应用 BWT 的输入字符串。
///
/// # Returns
/// # 返回值
///
/// A tuple containing:
/// 一个包含：
/// - The Burrows-Wheeler transformed string.
///   - Burrows-Wheeler 变换后的字符串。
/// - The index of the original string in the sorted rotations.
///   - 原始字符串在排序旋转表中的索引。
///
/// # Examples
/// # 示例
///
/// ```
/// let (encoded, index) = burrows_wheeler_transform("banana".to_owned());
/// assert_eq!(encoded, "annb$aa");
/// assert_eq!(index, 4);
/// ```
///
/// # Complexity
/// # 复杂度
///
/// The time complexity of this function is O(n^2 * log n), where n is the length of the input string.
/// This is because sorting the rotations takes O(n^2 * log n) time.
/// 此函数的时间复杂度为 O(n^2 * log n)，其中 n 是输入字符串的长度。排序旋转表需要 O(n^2 * log n) 的时间。
///
/// The space complexity is O(n^2), due to the space used for the table of rotations.
/// 空间复杂度为 O(n^2)，因为需要用于存储旋转字符串表的空间。

pub fn burrows_wheeler_transform(input: String) -> (String, usize) {
  let len = input.len();
  // Create a vector with pre-allocated capacity for storing the table of rotated strings
  // 创建一个容量预分配的字符串向量，用于存储旋转字符串表
  let mut table = Vec::<String>::with_capacity(len);

  // Generate rotations and populate the table
  // 生成旋转字符串并填充表格
  for i in 0..len {
    table.push(input[i..].to_owned() + &input[..i]);
  }

  // Sort the rotations lexicographically
  // 对旋转字符串表进行词典排序
  table.sort_by_key(|a| a.to_lowercase());

  // Build the transformed string and find the index of the original string
  // 构建变换后的字符串和找到原始字符串的索引
  let mut encoded = String::new();
  let mut index: usize = 0;

  // Iterate over the table of rotated strings, enumerate them with their indices,
  // 对旋转字符串表进行迭代，同时为它们的索引加上枚举标识，
  // and take only the first `len` elements for further processing.
  // 并仅处理前 `len` 个元素以进行后续处理。
  for (i, item) in table.iter().enumerate().take(len) {
    encoded.push(item.chars().last().unwrap());

    // Check if the current rotated string matches the original input string
    // 检查当前旋转字符串是否与原始输入字符串匹配，
    // and store its index for later retrieval
    // 并将其索引存储以便稍后检索
    if item.eq(&input) {
      index = i;
    }
  }

  (encoded, index)
}

/// Reverses the Burrows-Wheeler Transform to retrieve the original string.
/// 反转 Burrows-Wheeler 变换，恢复原始字符串。
///
/// This function takes the result of the Burrows-Wheeler Transform, which consists of the transformed
/// string and the index of the original string, and returns the original string before the transformation.
/// 此函数接受 Burrows-Wheeler 变换的结果，其中包括变换后的字符串和索引，返回变换之前的原始字符串。
///
/// # Arguments
/// # 参数
///
/// * `input` - A tuple containing the transformed string and the index.
/// * `input` - 包含变换后的字符串和索引的元组。
///
/// # Returns
/// # 返回值
///
/// The original string before the Burrows-Wheeler Transform was applied.
/// Burrows-Wheeler 变换之前的原始字符串。
///
/// # Examples
/// # 示例
///
/// ```
/// let transformed = ("annb$aa".to_owned(), 4);
/// let original = inv_burrows_wheeler_transform(transformed);
/// assert_eq!(original, "banana");
/// ```
///
/// # Complexity
/// # 复杂度
///
/// The time complexity of this function is O(n^2), where n is the length of the transformed string.
/// This is because sorting the index table takes O(n^2) time.
/// 此函数的时间复杂度为 O(n^2)，其中 n 是变换后的字符串的长度。排序索引表需要 O(n^2) 的时间。
///
/// The space complexity is O(n^2), due to the space used for the index table.
/// 空间复杂度为 O(n^2)，因为需要用于存储索引表的空间。

pub fn inv_burrows_wheeler_transform(input: (String, usize)) -> String {
  let len = input.0.len();
  let mut table = Vec::<(usize, char)>::with_capacity(len);

  // Build the table of indices and characters
  // 构建索引表和字符表
  for i in 0..len {
    table.push((i, input.0.chars().nth(i).unwrap()));
  }

  // Sort the table based on characters
  // 根据字符对表进行排序
  table.sort_by(|a, b| a.1.cmp(&b.1));

  // Build the decoded string using the table of indices
  // 根据索引表构建解码后的字符串
  let mut decoded = String::new();
  let mut idx = input.1;

  for _ in 0..len {
    decoded.push(table[idx].1);
    idx = table[idx].0;
  }

  decoded
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT".to_string())),
      "CARROT"
    );
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO".to_string())),
      "TOMATO"
    );
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST".to_string())),
      "THISISATEST"
    );
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS".to_string())),
      "THEALGORITHMS"
    );
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST".to_string())),
      "RUST"
    );
  }

  #[test]
  fn special_characters() {
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::".to_string())),
      "!.!.!??.=::"
    );
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("!{}{}(((&&%%!??.=::".to_string())),
      "!{}{}(((&&%%!??.=::"
    );
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]".to_string())),
      "//&$[]"
    );
  }

  #[test]
  fn empty() {
    assert_eq!(
      inv_burrows_wheeler_transform(burrows_wheeler_transform("".to_string())),
      ""
    );
  }
}
