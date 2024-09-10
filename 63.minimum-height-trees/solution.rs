use std::collections::VecDeque;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        let mut degrees = vec![0; n];
        let graph = Self::build_graph(&mut degrees, n, &edges);
        let mut queue = VecDeque::<i32>::new();
        for (idx, &degree) in degrees.iter().enumerate() {
            if degree == 1 {
                queue.push_back(idx as i32);
            }
        }

        let mut last: Vec<i32> = vec![];
        while !queue.is_empty() {
            let mut leaves: Vec<i32> = vec![];
            for _ in 0..queue.len() {
                let cur = queue.pop_front().unwrap();
                leaves.push(cur);
                for &next in &graph[cur as usize] {
                    degrees[next as usize] -= 1;
                    if degrees[next as usize] == 1 {
                        queue.push_back(next);
                    }
                }
            }
            last = leaves;
        }
        last
    }

    fn build_graph(degrees: &mut Vec<u32>, n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let graph = {
            let mut graph: Vec<Vec<i32>> = vec![vec![]; n];
            for edge in edges {
                let u = edge[0];
                let v = edge[1];
                graph[u as usize].push(v);
                degrees[u as usize] += 1;
                graph[v as usize].push(u);
                degrees[v as usize] += 1;
            }
            graph
        };
        graph
    }
}
