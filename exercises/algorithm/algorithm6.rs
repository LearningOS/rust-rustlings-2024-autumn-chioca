/*
    dfs
    This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        if src >= self.adj.len() || dest >= self.adj.len() {
            panic!("Invalid edge from {} to {}", src, dest);
        }
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        if visited.contains(&v) {
            return; // 如果已经访问过，直接返回
        }
        visited.insert(v);
        visit_order.push(v);

        for &neighbor in &self.adj[v] {
            self.dfs_util(neighbor, visited, visit_order);
        }
    }

    fn dfs(&self, start: usize) -> Vec<usize> {
        if start >= self.adj.len() {
            panic!("Start vertex {} is out of bounds", start);
        }
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_edge() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 5); // 超出范围
    }

    #[test]
    #[should_panic]
    fn test_invalid_dfs_start() {
        let graph = Graph::new(3);
        graph.dfs(3); // 超出范围
    }
}
