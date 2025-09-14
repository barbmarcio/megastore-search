use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

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

    pub fn get_recommendations(&self, product_id: u64, limit: usize) -> Vec<(u64, f32)> {
        let connections = self.get_connections(product_id);

        let mut recommendations: Vec<(u64, f32)> = connections
            .into_iter()
            .map(|(id, weight, relation_type)| {
                let type_multiplier = match relation_type {
                    RelationType::BoughtTogether => 1.5,
                    RelationType::Similar => 1.3,
                    RelationType::SameBrand => 1.1,
                    RelationType::SameCategory => 1.0,
                };
                (id, weight * type_multiplier)
            })
            .collect();

        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        recommendations.truncate(limit);
        recommendations
    }

    pub fn get_recommendations_depth_2(&self, product_id: u64, limit: usize) -> Vec<(u64, f32)> {
        let mut scores: HashMap<u64, f32> = HashMap::new();
        let mut visited = HashSet::new();
        visited.insert(product_id);

        let direct_connections = self.get_connections(product_id);
        for (connected_id, weight, relation_type) in direct_connections {
            let type_multiplier = match relation_type {
                RelationType::BoughtTogether => 1.5,
                RelationType::Similar => 1.3,
                RelationType::SameBrand => 1.1,
                RelationType::SameCategory => 1.0,
            };

            let score = weight * type_multiplier;
            scores.insert(connected_id, score);
            visited.insert(connected_id);

            let second_level = self.get_connections(connected_id);
            for (second_id, second_weight, second_relation) in second_level {
                if !visited.contains(&second_id) {
                    let second_multiplier = match second_relation {
                        RelationType::BoughtTogether => 0.75,
                        RelationType::Similar => 0.65,
                        RelationType::SameBrand => 0.55,
                        RelationType::SameCategory => 0.5,
                    };

                    let second_score = score * 0.5 * second_weight * second_multiplier;
                    *scores.entry(second_id).or_insert(0.0) += second_score;
                }
            }
        }

        let mut recommendations: Vec<(u64, f32)> = scores.into_iter().collect();
        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        recommendations.truncate(limit);
        recommendations
    }

    pub fn get_similar_products(&self, product_id: u64) -> Vec<u64> {
        self.get_connections(product_id)
            .into_iter()
            .filter(|(_, _, relation_type)| *relation_type == RelationType::Similar)
            .map(|(id, _, _)| id)
            .collect()
    }

    pub fn get_frequently_bought_together(&self, product_id: u64) -> Vec<u64> {
        self.get_connections(product_id)
            .into_iter()
            .filter(|(_, _, relation_type)| *relation_type == RelationType::BoughtTogether)
            .map(|(id, _, _)| id)
            .collect()
    }
}