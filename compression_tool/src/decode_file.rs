use std::{char, collections::VecDeque};

struct Node {
    character: char,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>
}

impl Node {
    fn new(character: char) -> Node {
        Node {
            character,
            left_child: None,
            right_child: None
        }
    }
}

fn deserialize_huffman_binary_tree(serialized_huffman_binary_tree: &str) -> Option<Box<Node>> {
    let nodes: Vec<&str> = serialized_huffman_binary_tree.split(" ").collect();
    
    if nodes.is_empty() {
        return None;
    }

    let mut queue = VecDeque::new();

    let mut root = Box::new(Node::new('\0'));
    queue.push_back(&mut root);
    let mut i = 0;
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if i + 1 < nodes.len() && nodes[i + 1] != "null" {
            let char = if nodes[i + 1].is_empty() {
                '\0'
            } else {
                nodes[i + 1].chars().next().unwrap()
            };
            let left_child = Box::new(Node::new(char));
            node.left_child = Some(left_child);
            queue.push_back(node.left_child.as_mut().unwrap());
        }
        i += 1;
        if i + 1 < nodes.len() && nodes[i + 1] != "null" {
            let char = if nodes[i + 1].is_empty() {
                '\0'
            } else {
                nodes[i + 1].chars().next().unwrap()
            };
            let right_child = Box::new(Node::new(char));
            node.right_child = Some(right_child);
            queue.push_back(node.right_child.as_mut().unwrap());
        }
        i += 1;
    }

    Some(root)
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_huffman_binary_tree() {
        let serialized_huffman_binary_tree = " e  null null   u l d  null null null null null null c  null null  m z k null null null null null null";
        let root = deserialize_huffman_binary_tree (serialized_huffman_binary_tree);
    }
}