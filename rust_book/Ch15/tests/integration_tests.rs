//! Chapter 15 Integration Tests
//!
//! These tests demonstrate how smart pointer concepts integrate
//! with real-world patterns and mission requirements.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[test]
fn test_binary_tree_with_box() {
    // Integration with Mission 3 - Binary Search Tree
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }

    impl TreeNode {
        fn new(value: i32) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }

        fn insert(&mut self, value: i32) {
            if value < self.value {
                match &mut self.left {
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                    Some(left) => left.insert(value),
                }
            } else {
                match &mut self.right {
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                    Some(right) => right.insert(value),
                }
            }
        }
    }

    let mut tree = TreeNode::new(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    assert_eq!(tree.value, 5);
    assert_eq!(tree.left.as_ref().unwrap().value, 3);
    assert_eq!(tree.right.as_ref().unwrap().value, 7);
}

#[test]
fn test_linked_list_with_rc_refcell() {
    // Integration with Mission 4 - Doubly Linked List pattern
    type Link = Option<Rc<RefCell<Node>>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Link,
        prev: Weak<RefCell<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                value,
                next: None,
                prev: Weak::new(),
            }))
        }
    }

    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);

    // Link forward
    node1.borrow_mut().next = Some(Rc::clone(&node2));
    node2.borrow_mut().next = Some(Rc::clone(&node3));

    // Link backward (using Weak to prevent cycles)
    node2.borrow_mut().prev = Rc::downgrade(&node1);
    node3.borrow_mut().prev = Rc::downgrade(&node2);

    // Traverse forward
    assert_eq!(node1.borrow().value, 1);
    let next = node1.borrow().next.as_ref().unwrap().clone();
    assert_eq!(next.borrow().value, 2);

    // Traverse backward
    let prev = node3.borrow().prev.upgrade().unwrap();
    assert_eq!(prev.borrow().value, 2);
}

#[test]
fn test_graph_structure_with_weak() {
    // Graph nodes with Weak references to prevent cycles
    #[derive(Debug)]
    struct GraphNode {
        _id: usize,
        neighbors: RefCell<Vec<Weak<GraphNode>>>,
    }

    impl GraphNode {
        fn new(id: usize) -> Rc<GraphNode> {
            Rc::new(GraphNode {
                _id: id,
                neighbors: RefCell::new(vec![]),
            })
        }

        fn add_neighbor(&self, neighbor: &Rc<GraphNode>) {
            self.neighbors.borrow_mut().push(Rc::downgrade(neighbor));
        }
    }

    let node1 = GraphNode::new(1);
    let node2 = GraphNode::new(2);
    let node3 = GraphNode::new(3);

    // Create connections (bidirectional graph)
    node1.add_neighbor(&node2);
    node1.add_neighbor(&node3);
    node2.add_neighbor(&node1);
    node2.add_neighbor(&node3);
    node3.add_neighbor(&node1);

    // Verify connections without memory leaks
    assert_eq!(node1.neighbors.borrow().len(), 2);
    assert_eq!(node2.neighbors.borrow().len(), 2);
    assert_eq!(node3.neighbors.borrow().len(), 1);
}

#[test]
fn test_observer_pattern_with_rc() {
    // Observer pattern using Rc for shared subjects
    trait Observer {
        fn update(&self, value: i32);
    }

    struct Subject {
        observers: RefCell<Vec<Rc<dyn Observer>>>,
        value: RefCell<i32>,
    }

    impl Subject {
        fn new() -> Rc<Subject> {
            Rc::new(Subject {
                observers: RefCell::new(vec![]),
                value: RefCell::new(0),
            })
        }

        fn attach(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(observer);
        }

        fn set_value(&self, value: i32) {
            *self.value.borrow_mut() = value;
            self.notify();
        }

        fn notify(&self) {
            let value = *self.value.borrow();
            for observer in self.observers.borrow().iter() {
                observer.update(value);
            }
        }
    }

    struct ConcreteObserver {
        _id: usize,
        last_value: RefCell<i32>,
    }

    impl Observer for ConcreteObserver {
        fn update(&self, value: i32) {
            *self.last_value.borrow_mut() = value;
        }
    }

    let subject = Subject::new();

    let observer1 = Rc::new(ConcreteObserver {
        _id: 1,
        last_value: RefCell::new(0),
    });

    let observer2 = Rc::new(ConcreteObserver {
        _id: 2,
        last_value: RefCell::new(0),
    });

    subject.attach(Rc::clone(&observer1) as Rc<dyn Observer>);
    subject.attach(Rc::clone(&observer2) as Rc<dyn Observer>);

    subject.set_value(42);

    assert_eq!(*observer1.last_value.borrow(), 42);
    assert_eq!(*observer2.last_value.borrow(), 42);
}

#[test]
fn test_cache_with_rc() {
    // Shared cache using Rc for read-only data
    struct Cache {
        data: Rc<Vec<String>>,
    }

    impl Cache {
        fn new(data: Vec<String>) -> Self {
            Cache {
                data: Rc::new(data),
            }
        }

        fn get_shared(&self) -> Rc<Vec<String>> {
            Rc::clone(&self.data)
        }
    }

    let cache = Cache::new(vec![
        String::from("item1"),
        String::from("item2"),
        String::from("item3"),
    ]);

    let shared1 = cache.get_shared();
    let shared2 = cache.get_shared();

    assert_eq!(Rc::strong_count(&cache.data), 3);
    assert_eq!(shared1.len(), 3);
    assert_eq!(shared2[0], "item1");
}
