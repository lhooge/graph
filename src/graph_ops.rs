use crate::graph::{DirectedCSRGraph, NodeLabeledCSRGraph, UndirectedCSRGraph};
use crate::index::Idx;
use crate::{DirectedGraph, DirectedGraphOps, UndirectedGraph, UndirectedGraphOps};
use std::ops::Range;

pub(crate) fn node_map_partition<Node: Idx, F>(
    node_map: F,
    node_count: Node,
    batch_size: Node,
) -> Vec<Range<Node>>
where
    F: Fn(Node) -> Node,
{
    let mut partitions = Vec::new();

    let mut partition_size = Node::zero();
    let mut partition_start = Node::zero();
    for i in 0..node_count.index() {
        partition_size += node_map(Node::new(i));

        if partition_size >= batch_size || i == node_count.index() - 1 {
            partitions.push(partition_start..Node::new(i));
            partition_size = Node::zero();
            partition_start = Node::new(i + 1);
        }
    }

    partitions
}

impl<Node: Idx> DirectedGraphOps<Node> for DirectedCSRGraph<Node> {}
impl<Node: Idx> UndirectedGraphOps<Node> for UndirectedCSRGraph<Node> {}
impl<Node: Idx, G: DirectedGraph<Node>> DirectedGraphOps<Node> for NodeLabeledCSRGraph<G> {}
impl<Node: Idx, G: UndirectedGraph<Node>> UndirectedGraphOps<Node> for NodeLabeledCSRGraph<G> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_map_1_partition() {
        let partitions = node_map_partition(|_| 1_usize, 10, 10);
        assert_eq!(partitions.len(), 1);
        assert_eq!(partitions[0], 0..9);
    }

    #[test]
    fn node_map_2_partitions() {
        let partitions = node_map_partition(|x| x % 2_usize, 10, 4);
        assert_eq!(partitions.len(), 2);
        assert_eq!(partitions[0], 0..7);
        assert_eq!(partitions[1], 8..9);
    }

    #[test]
    fn node_map_6_partitions() {
        let partitions = node_map_partition(|x| x as usize, 10, 6);
        assert_eq!(partitions.len(), 6);
        assert_eq!(partitions[0], 0..3);
        assert_eq!(partitions[1], 4..5);
        assert_eq!(partitions[2], 6..6);
        assert_eq!(partitions[3], 7..7);
        assert_eq!(partitions[4], 8..8);
        assert_eq!(partitions[5], 9..9);
    }
}
