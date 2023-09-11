use std::collections::HashSet;
use std::collections::VecDeque;

/// 使用深度优先搜索算法在图中查找从根节点到目标节点的路径。
///
/// This function implements the Depth-First Search (DFS) algorithm to find a path from the root vertex
/// to the target vertex within the given graph. The algorithm starts at the root vertex and explores
/// as far as possible along each branch before backtracking. It returns the history of visited vertices
/// in the form of a `Vec<u32>` if a path to the target vertex is found, or `None` if no path exists.
///
/// # 参数 (Parameters)
///
/// - `graph`: 表示图的数据结构，应该包含顶点和边的信息。
///   (Represents the data structure of the graph, including information about vertices and edges.)
/// - `root`: 开始搜索的起始顶点。
///   (The starting vertex for the search.)
/// - `objective`: 要查找的目标顶点。
///   (The target vertex to find.)
///
/// # 返回值 (Returns)
///
/// 如果找到了从根节点到目标节点的路径，将返回一个包含访问历史的 `Vec<u32>`，表示经过的顶点值的顺序。如果未找到路径，则返回 `None`。
/// (If a path from the root vertex to the target vertex is found, it returns a `Some` containing a `Vec<u32>`
/// representing the order of visited vertices. If no path exists, it returns `None`.)
///
/// # 示例 (Examples)
///
/// ```rust
/// use std::collections::HashSet;
/// use std::collections::VecDeque;
///
/// struct Vertex {
///     // 顶点的值
///     value: u32,
///     // 其他顶点的集合，表示与该顶点相邻的顶点
///     neighbors: HashSet<Vertex>,
/// }
///
/// struct Graph {
///     vertices: Vec<Vertex>,
/// }
///
/// // 创建一个图对象
/// let graph = Graph { vertices: vec![/* 初始化顶点 */] };
///
/// // 指定起始顶点和目标顶点
/// let root_vertex = /* 起始顶点 */;
/// let objective_vertex = /* 目标顶点 */;
///
/// // 使用深度优先搜索算法查找路径
/// let result = depth_first_search(&graph, root_vertex, objective_vertex);
///
/// match result {
///     Some(history) => {
///         println!("找到路径: {:?}", history);
///     },
///     None => {
///         println!("未找到路径");
///     },
/// }
/// ```
///
/// # 复杂度分析 (Complexity Analysis)
///
/// - 时间复杂度：在最坏情况下，如果需要遍历整个图，时间复杂度为 O(V + E)，其中 V 为顶点数，E 为边数。
///   (Time Complexity: In the worst case, where the entire graph needs to be traversed, the time complexity
///   is O(V + E), where V is the number of vertices and E is the number of edges.)
/// - 空间复杂度：空间复杂度为 O(V)，其中 V 为顶点数，用于存储已访问的顶点集合。
///   (Space Complexity: The space complexity is O(V), where V is the number of vertices, used to store the set
///   of visited vertices.)
///
pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u32>> {
  let mut visited: HashSet<Vertex> = HashSet::new();
  let mut history: Vec<u32> = Vec::new();
  let mut queue = VecDeque::new();

  queue.push_back(root);

  // 在队列中有元素的情况下循环
  // Loop while there is an element in the queue.
  while let Some(current_vertex) = queue.pop_front() {
    // 将当前顶点添加到已访问的顶点历史中
    // Add the current vertex to the history of visited vertices.
    history.push(current_vertex.value());

    // 检查当前顶点是否是目标
    // Check if the current vertex is the objective.
    if current_vertex == objective {
      // 返回包含已访问顶点历史的可选项
      // Return an Option with the history of visited vertices.
      return Some(history);
    }

    // 遍历当前顶点的邻居
    // Iterate over the neighbors of the current vertex.
    for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
      // 如果该值尚未存在，则插入已访问的顶点集合
      // Insert into the set of visited vertices if this value does not exist yet.
      if visited.insert(neighbor) {
        // 将邻居添加到队列的前面
        // Add the neighbor to the front of the queue.
        queue.push_front(neighbor);
      }
    }
  }

  // 如果所有顶点都已访问且未找到目标返回一个包含 None 值的可选项
  // If all vertices are visited and the objective is not found, return an Option with None.
  None
}

// Data Structures
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
  vertices: Vec<Vertex>,
  edges: Vec<Edge>,
}

impl Graph {
  pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
    Graph { vertices, edges }
  }
}

impl From<u32> for Vertex {
  fn from(item: u32) -> Self {
    Vertex(item)
  }
}

impl Vertex {
  pub fn value(&self) -> u32 {
    self.0
  }
  pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
    graph
      .edges
      .iter()
      .filter(|e| e.0 == self.0)
      .map(|e| e.1.into())
      .collect()
  }
}

impl From<(u32, u32)> for Edge {
  fn from(item: (u32, u32)) -> Self {
    Edge(item.0, item.1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn gen_graph(edges: Vec<(u32, u32)>, root: u32, objective: u32) -> Graph {
    let vertices = vec![1, 2, 3, 4, 5, 6, 7];
    let edges = edges;
    let root = root;
    let objective = objective;

    let graph = Graph::new(
      vertices.into_iter().map(|v| v.into()).collect(),
      edges.into_iter().map(|e| e.into()).collect(),
    );

    return graph;
  }

  #[test]
  fn find_1_fail() {
    let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
    let root = 1;
    let objective = 99;
    let graph = gen_graph(edges, root, objective);
    assert_eq!(
      depth_first_search(&graph, root.into(), objective.into()),
      None
    );
  }

  #[test]
  fn find_1_sucess() {
    let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
    let root = 1;
    let objective = 7;
    let correct_path = vec![1, 2, 4, 5, 3, 6, 7];
    let graph = gen_graph(edges, root, objective);
    assert_eq!(
      depth_first_search(&graph, root.into(), objective.into()),
      Some(correct_path)
    );
  }

  #[test]
  fn find_2_sucess() {
    let edges = vec![
      (0, 1),
      (1, 3),
      (3, 2),
      (2, 1),
      (3, 4),
      (4, 5),
      (5, 7),
      (7, 6),
      (6, 4),
    ];
    let root = 0;
    let objective = 6;
    let correct_path = vec![0, 1, 3, 2, 4, 5, 7, 6];
    let graph = gen_graph(edges, root, objective);
    assert_eq!(
      depth_first_search(&graph, root.into(), objective.into()),
      Some(correct_path)
    );
  }

  #[test]
  fn find_3_sucess() {
    let edges = vec![
      (0, 1),
      (1, 3),
      (3, 2),
      (2, 1),
      (3, 4),
      (4, 5),
      (5, 7),
      (7, 6),
      (6, 4),
    ];
    let root = 0;
    let objective = 4;
    let correct_path = vec![0, 1, 3, 2, 4];
    let graph = gen_graph(edges, root, objective);
    assert_eq!(
      depth_first_search(&graph, root.into(), objective.into()),
      Some(correct_path)
    );
  }
}

fn main() {}
