//! Priority queue implementation optimized for pathfinding algorithms

use crate::{NodeId, Weight};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A node in the priority queue with its distance/cost
#[derive(Debug, Clone, PartialEq)]
pub struct QueueNode {
    pub node: NodeId,
    pub cost: Weight,
}

impl QueueNode {
    pub fn new(node: NodeId, cost: Weight) -> Self {
        Self { node, cost }
    }
}

// Implement ordering for BinaryHeap (min-heap behavior)
impl Eq for QueueNode {}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // For BinaryHeap to work as min-heap, we need to reverse ordering
        // BinaryHeap is max-heap by default, so we want higher cost to be "less"
        // This way BinaryHeap will pop lower costs first
        match other.cost.partial_cmp(&self.cost) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Greater) => Ordering::Greater,
            Some(Ordering::Equal) => {
                // Costs are equal, break ties with node IDs (lower node ID first)
                other.node.cmp(&self.node)
            }
            // Handle NaN cases by using node ID comparison
            None => other.node.cmp(&self.node),
        }
    }
}

/// Priority queue trait for pathfinding algorithms
pub trait PathfindingQueue {
    /// Add a node with its priority/cost to the queue
    fn push(&mut self, node: NodeId, cost: Weight);

    /// Remove and return the node with lowest cost
    fn pop(&mut self) -> Option<QueueNode>;

    /// Check if the queue is empty
    fn is_empty(&self) -> bool;

    /// Get the number of items in the queue
    fn len(&self) -> usize;

    /// Clear all items from the queue
    fn clear(&mut self);
}

/// Binary heap implementation of priority queue
#[derive(Debug, Clone)]
pub struct BinaryHeapQueue {
    heap: BinaryHeap<QueueNode>,
}

impl BinaryHeapQueue {
    /// Create a new empty priority queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    /// Create a priority queue with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }
}

impl Default for BinaryHeapQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl PathfindingQueue for BinaryHeapQueue {
    fn push(&mut self, node: NodeId, cost: Weight) {
        self.heap.push(QueueNode::new(node, cost));
    }

    fn pop(&mut self) -> Option<QueueNode> {
        self.heap.pop()
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }

    fn clear(&mut self) {
        self.heap.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_node_ordering() {
        let node1 = QueueNode::new(0, 5.0);
        let node2 = QueueNode::new(1, 3.0);
        let node3 = QueueNode::new(2, 3.0);

        // Due to reversed ordering for BinaryHeap min-heap behavior:
        // Higher cost nodes should be "less" in direct comparison
        // This ensures BinaryHeap pops lower costs first
        assert!(node1 < node2); // node1 (cost 5.0) < node2 (cost 3.0) for BinaryHeap

        // Same cost, higher node ID should be "less" for BinaryHeap
        assert!(node3 < node2); // node3 (id 2) < node2 (id 1) for BinaryHeap
    }

    #[test]
    fn test_binary_heap_queue() {
        let mut queue = BinaryHeapQueue::new();

        // Test empty queue
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert!(queue.pop().is_none());

        // Add nodes with different costs
        queue.push(0, 5.0);
        queue.push(1, 2.0);
        queue.push(2, 8.0);
        queue.push(3, 2.0);

        assert_eq!(queue.len(), 4);
        assert!(!queue.is_empty());

        // Should pop nodes in cost order (lowest first)
        let node = queue.pop().unwrap();
        assert_eq!(node.cost, 2.0);
        // Node ID should break ties (1 < 3)
        assert_eq!(node.node, 1);

        let node = queue.pop().unwrap();
        assert_eq!(node.cost, 2.0);
        assert_eq!(node.node, 3);

        let node = queue.pop().unwrap();
        assert_eq!(node.cost, 5.0);
        assert_eq!(node.node, 0);

        let node = queue.pop().unwrap();
        assert_eq!(node.cost, 8.0);
        assert_eq!(node.node, 2);

        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_clear() {
        let mut queue = BinaryHeapQueue::new();

        queue.push(0, 1.0);
        queue.push(1, 2.0);
        queue.push(2, 3.0);

        assert_eq!(queue.len(), 3);

        queue.clear();

        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert!(queue.pop().is_none());
    }

    #[test]
    fn test_queue_with_capacity() {
        let mut queue = BinaryHeapQueue::with_capacity(100);

        assert!(queue.is_empty());

        // Add many items to test capacity
        for i in 0..50 {
            queue.push(i, i as f64);
        }

        assert_eq!(queue.len(), 50);

        // Should still pop in correct order
        for i in 0..50 {
            let node = queue.pop().unwrap();
            assert_eq!(node.node, i);
            assert_eq!(node.cost, i as f64);
        }
    }
}
