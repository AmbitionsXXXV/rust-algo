mod bellman_ford;

pub mod dijsktra {
  use std::cmp::Reverse;
  use std::collections::{BTreeMap, BinaryHeap};
  use std::ops::Add;

  use crate::bellman_ford::bellman_ford::Graph;

  /// Dijkstra's Algorithm
  ///
  /// Dijkstra's algorithm is a shortest-path algorithm that finds the shortest path
  /// between a given start vertex and all other vertices in a weighted graph.
  ///
  /// It maintains a priority queue (binary heap) of vertices to visit, initially containing
  /// the start vertex with a distance of 0. The algorithm repeatedly selects the vertex
  /// with the smallest distance from the priority queue and explores its neighbors to update
  /// their distances. It uses a data structure (BTreeMap) to keep track of the shortest
  /// distances and predecessors for each vertex.
  ///
  /// 戴克斯特拉算法
  ///
  /// 戴克斯特拉算法是一种用于在带权图中找到给定起始顶点与所有其他顶点之间的最短路径的算法。
  ///
  /// 它维护一个优先队列（二叉堆），最初包含距离为0的起始顶点。该算法反复选择优先队列中距离最小的顶点，
  /// 并探索其相邻顶点以更新它们的距离。它使用数据结构（BTreeMap）来跟踪每个顶点的最短距离和前驱顶点。
  ///
  /// # Arguments
  ///
  /// - `graph`: The graph represented as an adjacency list.（以邻接表表示的图。）
  /// - `start`: The starting vertex from which to find shortest paths.（要查找最短路径的起始顶点。）
  ///
  /// # Returns
  ///
  /// A `BTreeMap` containing the shortest distances and predecessors for each vertex.
  /// （包含每个顶点的最短距离和前驱顶点的 `BTreeMap`。）
  ///
  /// # Complexity
  ///
  /// The time complexity of this algorithm is O((V + E) * log(V)), where V is the number
  /// of vertices and E is the number of edges in the graph. This is due to the use of a binary
  /// heap for priority queue operations, which has a time complexity of O(log(V)).
  /// （该算法的时间复杂度为O((V + E) * log(V))，其中V是顶点数，E是图中的边数。这是由于使用二叉堆进行优先队列操作，
  /// 二叉堆的时间复杂度为O(log(V))。）
  ///
  /// The space complexity is O(V), where V is the number of vertices, as we store the shortest
  /// distances and predecessors for each vertex in a `BTreeMap`.
  /// （空间复杂度为O(V)，其中V是顶点数，因为我们在 `BTreeMap` 中存储了每个顶点的最短距离和前驱顶点。）
  ///
  /// # Example
  ///
  /// ```
  /// use std::collections::BTreeMap;
  /// use dijkstra::dijkstra;
  ///
  /// let mut graph = BTreeMap::new();
  /// graph.insert(1, vec![(2, 5), (3, 2)]);
  /// graph.insert(2, vec![(3, 1)]);
  /// graph.insert(3, vec![(4, 4)]);
  ///
  /// let start = 1;
  /// let result = dijkstra(&graph, &start);
  ///
  /// println!("{:?}", result);
  /// ```
  pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: &V,
  ) -> BTreeMap<V, Option<(V, E)>> {
    // 创建结果映射表，用于存储最短路径和权重
    // Create a result map to store the shortest paths and their weights.
    let mut ans = BTreeMap::new();
    // 创建优先队列，用于按距离排序的节点
    // Create a priority queue to order nodes by distance.
    let mut prio = BinaryHeap::new();

    // 起始节点是特殊情况，没有前驱节点
    // The start node is a special case and has no predecessor.
    ans.insert(*start, None);

    // 将起始节点相邻的节点加入结果和优先队列
    // Add neighboring nodes of the start node to the result and priority queue.
    for (new, weight) in &graph[start] {
      ans.insert(*new, Some((*start, *weight)));
      prio.push(Reverse((*weight, new, start)));
    }

    // 主循环，直到优先队列为空
    // Main loop, continues until the priority queue is empty.
    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
      match ans[new] {
        // 如果优先队列中的节点已经在结果中，跳过计算
        // If the node from the priority queue is already in the result, skip the computation.
        Some((p, d)) if p == *prev && d == dist_new => {}
        // 否则，继续处理下一个节点
        // Otherwise, continue to the next node.
        _ => continue,
      }

      // 遍历当前节点相邻的节点
      // Iterate through the neighbors of the current node.
      for (next, weight) in &graph[new] {
        match ans.get(next) {
          // 如果结果中已经包含了更短的路径，跳过
          // If a shorter path to the node is already in the result, skip it.
          Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
          // 如果结果中没有节点，表示新节点是起始节点，不会再次加入优先队列
          // If the node is not in the result (None), it means it's the start node, and it won't be added to the priority queue again.
          Some(None) => {}
          // 否则，更新最短路径和加入优先队列
          // Otherwise, update the shortest path and add it to the priority queue.
          _ => {
            ans.insert(*next, Some((*new, *weight + dist_new)));
            prio.push(Reverse((*weight + dist_new, next, new)));
          }
        }
      }
    }

    // 返回最终的结果映射表
    // Return the final result map.
    ans
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;

  use crate::bellman_ford::bellman_ford::{add_edge, Graph};
  use crate::dijsktra::dijkstra;

  #[test]
  fn single_vertex() {
    let mut graph: Graph<usize, usize> = BTreeMap::new();
    graph.insert(0, BTreeMap::new());

    let mut dists = BTreeMap::new();
    dists.insert(0, None);

    assert_eq!(dijkstra(&graph, &0), dists);
  }

  #[test]
  fn single_edge() {
    let mut graph = BTreeMap::new();
    add_edge(&mut graph, 0, 1, 2);

    let mut dists_0 = BTreeMap::new();
    dists_0.insert(0, None);
    dists_0.insert(1, Some((0, 2)));

    assert_eq!(dijkstra(&graph, &0), dists_0);

    let mut dists_1 = BTreeMap::new();
    dists_1.insert(1, None);

    assert_eq!(dijkstra(&graph, &1), dists_1);
  }

  #[test]
  fn tree_1() {
    let mut graph = BTreeMap::new();
    let mut dists = BTreeMap::new();

    dists.insert(1, None);

    for i in 1..100 {
      add_edge(&mut graph, i, i * 2, i * 2);
      add_edge(&mut graph, i, i * 2 + 1, i * 2 + 1);

      match dists[&i] {
        Some((_, d)) => {
          dists.insert(i * 2, Some((i, d + i * 2)));
          dists.insert(i * 2 + 1, Some((i, d + i * 2 + 1)));
        }
        None => {
          dists.insert(i * 2, Some((i, i * 2)));
          dists.insert(i * 2 + 1, Some((i, i * 2 + 1)));
        }
      }
    }

    assert_eq!(dijkstra(&graph, &1), dists);
  }

  #[test]
  fn graph_1() {
    let mut graph = BTreeMap::new();
    add_edge(&mut graph, 'a', 'c', 12);
    add_edge(&mut graph, 'a', 'd', 60);
    add_edge(&mut graph, 'b', 'a', 10);
    add_edge(&mut graph, 'c', 'b', 20);
    add_edge(&mut graph, 'c', 'd', 32);
    add_edge(&mut graph, 'e', 'a', 7);

    let mut dists_a = BTreeMap::new();
    dists_a.insert('a', None);
    dists_a.insert('c', Some(('a', 12)));
    dists_a.insert('d', Some(('c', 44)));
    dists_a.insert('b', Some(('c', 32)));

    assert_eq!(dijkstra(&graph, &'a'), dists_a);

    let mut dists_b = BTreeMap::new();
    dists_b.insert('b', None);
    dists_b.insert('a', Some(('b', 10)));
    dists_b.insert('c', Some(('a', 22)));
    dists_b.insert('d', Some(('c', 54)));

    assert_eq!(dijkstra(&graph, &'b'), dists_b);

    let mut dists_c = BTreeMap::new();
    dists_c.insert('c', None);
    dists_c.insert('b', Some(('c', 20)));
    dists_c.insert('d', Some(('c', 32)));
    dists_c.insert('a', Some(('b', 30)));

    assert_eq!(dijkstra(&graph, &'c'), dists_c);

    let mut dists_d = BTreeMap::new();
    dists_d.insert('d', None);

    assert_eq!(dijkstra(&graph, &'d'), dists_d);

    let mut dists_e = BTreeMap::new();
    dists_e.insert('e', None);
    dists_e.insert('a', Some(('e', 7)));
    dists_e.insert('c', Some(('a', 19)));
    dists_e.insert('d', Some(('c', 51)));
    dists_e.insert('b', Some(('c', 39)));

    assert_eq!(dijkstra(&graph, &'e'), dists_e);
  }
}

fn main() {}
