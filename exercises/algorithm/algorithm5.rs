/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//I AM NOT DONE
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO

        // 记录节点是否已被访问
        let mut visited = vec![false; self.adj.len()];
        // 使用队列来管理BFS的节点顺序
        let mut queue = VecDeque::new();
        // 存储访问节点的顺序
        let mut visit_order = vec![];

        // 初始化：标记起始节点为已访问，加入队列
        visited[start] = true;
        queue.push_back(start);

        // 当队列不为空时，继续BFS
        while let Some(node) = queue.pop_front() {
            // 记录当前节点的访问顺序
            visit_order.push(node);

            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[node] {
                // 如果邻接节点未被访问，则标记为已访问并加入队列
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

