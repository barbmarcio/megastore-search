use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProductNode {
    pub product_id: u64,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct EdgeWeight {
    pub weight: f32,
    pub relation_type: RelationType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationType {
    Similar,
    BoughtTogether,
    SameCategory,
    SameBrand,
}

pub struct RecommendationGraph {
    graph: UnGraph<ProductNode, EdgeWeight>,
    product_to_node: HashMap<u64, NodeIndex>,
}

impl RecommendationGraph {
    pub fn new() -> Self {
        RecommendationGraph {
            graph: UnGraph::new_undirected(),
            product_to_node: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product_id: u64, category: String) -> NodeIndex {
        if let Some(&node_index) = self.product_to_node.get(&product_id) {
            return node_index;
        }

        let node = ProductNode {
            product_id,
            category,
        };

        let node_index = self.graph.add_node(node);
        self.product_to_node.insert(product_id, node_index);
        node_index
    }

    pub fn get_node_index(&self, product_id: u64) -> Option<NodeIndex> {
        self.product_to_node.get(&product_id).copied()
    }

    pub fn product_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn get_product_node(&self, product_id: u64) -> Option<&ProductNode> {
        self.product_to_node
            .get(&product_id)
            .and_then(|&idx| self.graph.node_weight(idx))
    }
}