use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub struct Node {
    weight: i32,
    ch: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new_leaf(ch: char, weight: i32) -> Node {
        Node {
            ch,
            weight,
            left: None,
            right: None,
        }
    }

    fn new_internal(ch: char, weight: i32, left: Node, right: Node) -> Node {
        Node {
            ch,
            weight,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

pub fn generate_huffman_tree(freq: &HashMap<char, i32>) -> Node {
    let mut pq = BinaryHeap::new();

    for (&ch, &weight) in freq.iter() {
        pq.push(Node::new_leaf(ch, weight));
    }

    while pq.len() > 1 {
        let left = pq.pop().unwrap();
        let right = pq.pop().unwrap();

        let sum = left.weight + right.weight;
        pq.push(Node::new_internal('\0', sum, left, right));
    }

    pq.pop().unwrap()
}

pub fn inorder(root: &Option<Box<Node>>) {
    if let Some(node) = root {
        if node.ch != '\0' {
            println!("{}", node.ch);
        }
        inorder(&node.left);
        inorder(&node.right);
    }
}