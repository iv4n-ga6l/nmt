use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: u32,
    node: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make BinaryHeap a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Network {
    /// Finds the shortest path between two nodes using Dijkstra's algorithm.
    /// Returns the total weight of the shortest path and the sequence of nodes in the path.
    pub fn shortest_path(&self, start: &str, end: &str) -> Option<(u32, Vec<String>)> {
        let mut distances: HashMap<String, u32> = HashMap::new();
        let mut predecessors: HashMap<String, String> = HashMap::new();
        let mut heap = BinaryHeap::new();

        // Initialize distances to all nodes as infinity, except the start node
        for node in self.adjacency_list.keys() {
            distances.insert(node.clone(), u32::MAX);
        }
        distances.insert(start.to_string(), 0);

        // Push the start node into the heap
        heap.push(State {
            cost: 0,
            node: start.to_string(),
        });

        while let Some(State { cost, node }) = heap.pop() {
            // If we've reached the target node, stop
            if node == end {
                let mut path = Vec::new();
                let mut current = end.to_string();

                while let Some(predecessor) = predecessors.get(&current) {
                    path.push(current.clone());
                    current = predecessor.clone();
                }
                path.push(start.to_string());
                path.reverse();

                return Some((cost, path));
            }

            // Skip if the cost is greater than the recorded distance
            if cost > *distances.get(&node).unwrap_or(&u32::MAX) {
                continue;
            }

            // Explore neighbors
            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for (neighbor, weight) in neighbors {
                    let next = State {
                        cost: cost + weight,
                        node: neighbor.clone(),
                    };

                    if next.cost < *distances.get(&neighbor).unwrap_or(&u32::MAX) {
                        // Update the distance and predecessor
                        distances.insert(neighbor.clone(), next.cost);
                        predecessors.insert(neighbor.clone(), node.clone());

                        // Push the neighbor into the heap
                        heap.push(next);
                    }
                }
            }
        }

        // If we reach here, there's no path from start to end
        None
    }
}