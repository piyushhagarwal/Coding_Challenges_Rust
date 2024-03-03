mod frequency_table;
mod huffman_binary_tree;
mod prefix_table;
mod encode_file;
mod decode_file;

pub use frequency_table::generate_frequency_table;
pub use huffman_binary_tree::generate_huffman_tree;
pub use prefix_table::generate_prefix_table;
pub use encode_file::encode_file;
// pub use decode_file::decode_file;