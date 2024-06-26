//! This module provides a variety of operations.
mod convex_hull;
mod graph_coloring;
mod hanoi;
mod huffman_encoding;
mod kmeans;
mod nqueens;
mod two_sum;

pub use self::convex_hull::convex_hull_graham;
pub use self::graph_coloring::color_graph;
pub use self::hanoi::hanoi;
pub use self::huffman_encoding::HuffmanDictionary;
pub use self::kmeans::{f32, f64};
pub use self::nqueens::nqueens;
pub use self::two_sum::two_sum;
