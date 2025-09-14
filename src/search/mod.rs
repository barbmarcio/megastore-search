use crate::models::{Product, Category};
use crate::indexing::ProductIndex;
use crate::graph::RecommendationGraph;

#[derive(Debug)]
pub struct SearchResult {
    pub product: Product,
    pub score: f64,
    pub match_type: MatchType,
}

#[derive(Debug, Clone)]
pub enum MatchType {
    ExactName,
    PartialName,
    Brand,
    Category,
    Tag,
    Recommendation,
}

pub struct SearchEngine {
    index: ProductIndex,
    graph: RecommendationGraph,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            index: ProductIndex::new(),
            graph: RecommendationGraph::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let product_id = product.id;
        let category_str = product.category.to_string();

        self.graph.add_product(product_id, category_str);
        self.index.add_product(product);
    }

    pub fn add_product_relation(&mut self, product_id_1: u64, product_id_2: u64, weight: f32, relation_type: crate::graph::RelationType) {
        self.graph.add_edge(product_id_1, product_id_2, weight, relation_type);
    }

    pub fn basic_search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();

        let name_matches = self.index.search_by_name(query);
        for id in name_matches {
            if let Some(product) = self.index.get_product(id) {
                let score = product.search_score(query);
                let match_type = if product.name.to_lowercase() == query.to_lowercase() {
                    MatchType::ExactName
                } else {
                    MatchType::PartialName
                };

                results.push(SearchResult {
                    product: product.clone(),
                    score,
                    match_type,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }

    pub fn search_by_category(&self, category: &Category) -> Vec<SearchResult> {
        let category_matches = self.index.search_by_category(category);
        let mut results = Vec::new();

        for id in category_matches {
            if let Some(product) = self.index.get_product(id) {
                results.push(SearchResult {
                    product: product.clone(),
                    score: 1.0 + product.rating as f64 / 10.0,
                    match_type: MatchType::Category,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }

    pub fn search_by_brand(&self, brand: &str) -> Vec<SearchResult> {
        let brand_matches = self.index.search_by_brand(brand);
        let mut results = Vec::new();

        for id in brand_matches {
            if let Some(product) = self.index.get_product(id) {
                results.push(SearchResult {
                    product: product.clone(),
                    score: 1.0 + product.rating as f64 / 10.0,
                    match_type: MatchType::Brand,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }

    pub fn get_product_count(&self) -> usize {
        self.index.product_count()
    }

    pub fn get_graph_stats(&self) -> (usize, usize) {
        (self.graph.product_count(), self.graph.edge_count())
    }

    pub fn get_product(&self, id: u64) -> Option<&Product> {
        self.index.get_product(id)
    }
}