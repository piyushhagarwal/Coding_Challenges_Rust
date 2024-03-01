use std::collections::{BinaryHeap, HashMap};

pub struct Node {
    character: char,
    frequency: i32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>
}

impl Node {
    fn new_leaf(character: char, frequency: i32) -> Node {
        Node {
            character,
            frequency,
            left_child: None,
            right_child: None
        }
    }

    fn new_internal(frequency: i32, left_child: Box<Node>, right_child: Box<Node>) -> Node {
        Node {
            character: '\0',
            frequency,
            left_child: Some(left_child),
            right_child: Some(right_child)
        }
    }
}

// Ord, PartialOrd, Eq and PartialEq are traits that allow us to compare instances of a type and are used by the BinaryHeap to order the nodes by their frequency  

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
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
        self.frequency == other.frequency
    }
}

// The generate_huffman_tree function receives a reference to a frequency table and returns a Box<Node> that represents the root of the Huffman tree generated
pub fn generate_huffman_tree(frequency_table: &HashMap<char, i32>) -> Box<Node> {
    let mut priority_queue = BinaryHeap::new(); // The nodes are ordered by their frequency

    // Create a leaf node for each character and add it to the priority queue
    for(&character, &frequency) in frequency_table.iter(){
        priority_queue.push(Box::new(Node::new_leaf(character, frequency)));
    }

    // While there is more than one node in the queue
    while priority_queue.len() > 1 {
        // Remove the two nodes of the highest priority (the lowest frequency) from the queue
        let left = priority_queue.pop().unwrap();
        let right = priority_queue.pop().unwrap();

        // Create a new internal node with these two nodes as children and with a frequency equal to the sum of their frequencies
        let new_node = Node::new_internal(left.frequency + right.frequency, left, right);

        // Add the new node to the queue
        priority_queue.push(Box::new(new_node));
    }

    // The remaining node is the root node of the Huffman tree generated
    priority_queue.pop().unwrap()

}


// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_huffman_tree() {
        let mut freq = HashMap::new();
        freq.insert('c', 32);
        freq.insert('d', 42);
        freq.insert('e', 120);
        freq.insert('k', 7);
        freq.insert('l', 42);
        freq.insert('m', 24);
        freq.insert('u', 37);
        freq.insert('z', 2);

        let root = generate_huffman_tree(&freq);
        
        // The value shouldn't be moved 
        assert_eq!(root.frequency, 306);
        assert_eq!(root.character, '\0');

        // The left child of the root should be a leaf node with character 'e' and frequency 120
        let left_child = root.left_child.unwrap();
        assert_eq!(left_child.frequency, 120);
        assert_eq!(left_child.character, 'e');

        // The right child of the root should be an internal node with frequency 186
        let right_child = root.right_child.unwrap();
        assert_eq!(right_child.frequency, 186);

        // The left child of the right child of the root should be a leaf node with character 'u' and frequency 37
        let left_child_of_right_child = right_child.left_child.unwrap();
        assert_eq!(left_child_of_right_child.frequency, 79);

        // The right child of the right child of the root should be an internal node with frequency 149
        let right_child_of_right_child = right_child.right_child.unwrap();
        assert_eq!(right_child_of_right_child.frequency, 107);
    }
}