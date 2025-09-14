use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
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

    pub fn add_edge(
        &mut self,
        product_id_1: u64,
        product_id_2: u64,
        weight: f32,
        relation_type: RelationType,
    ) -> bool {
        if let (Some(&node1), Some(&node2)) = (
            self.product_to_node.get(&product_id_1),
            self.product_to_node.get(&product_id_2),
        ) {
            let edge_weight = EdgeWeight {
                weight,
                relation_type,
            };
            self.graph.add_edge(node1, node2, edge_weight);
            true
        } else {
            false
        }
    }

    pub fn connect_similar_products(&mut self, product_id_1: u64, product_id_2: u64, similarity: f32) {
        self.add_edge(product_id_1, product_id_2, similarity, RelationType::Similar);
    }

    pub fn connect_bought_together(&mut self, product_id_1: u64, product_id_2: u64, frequency: f32) {
        self.add_edge(product_id_1, product_id_2, frequency, RelationType::BoughtTogether);
    }

    pub fn connect_same_category(&mut self, product_id_1: u64, product_id_2: u64) {
        self.add_edge(product_id_1, product_id_2, 0.5, RelationType::SameCategory);
    }

    pub fn connect_same_brand(&mut self, product_id_1: u64, product_id_2: u64) {
        self.add_edge(product_id_1, product_id_2, 0.6, RelationType::SameBrand);
    }

    pub fn get_connections(&self, product_id: u64) -> Vec<(u64, f32, RelationType)> {
        if let Some(&node_idx) = self.product_to_node.get(&product_id) {
            let mut connections = Vec::new();

            for edge in self.graph.edges(node_idx) {
                let target_node = edge.target();
                if let Some(target_product) = self.graph.node_weight(target_node) {
                    let edge_weight = edge.weight();
                    connections.push((
                        target_product.product_id,
                        edge_weight.weight,
                        edge_weight.relation_type.clone(),
                    ));
                }
            }

            connections
        } else {
            Vec::new()
        }
    }

    pub fn has_edge(&self, product_id_1: u64, product_id_2: u64) -> bool {
        if let (Some(&node1), Some(&node2)) = (
            self.product_to_node.get(&product_id_1),
            self.product_to_node.get(&product_id_2),
        ) {
            self.graph.find_edge(node1, node2).is_some()
        } else {
            false
        }
    }
}