use crate::models::{Product, Category};
use crate::indexing::ProductIndex;
use crate::graph::RecommendationGraph;
use std::collections::HashSet;

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
    Combined,
}

#[derive(Debug, Clone)]
pub struct SearchFilters {
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub min_rating: Option<f32>,
    pub category: Option<Category>,
    pub brand: Option<String>,
    pub tags: Vec<String>,
    pub in_stock_only: bool,
}

impl SearchFilters {
    pub fn new() -> Self {
        SearchFilters {
            min_price: None,
            max_price: None,
            min_rating: None,
            category: None,
            brand: None,
            tags: Vec::new(),
            in_stock_only: false,
        }
    }

    pub fn price_range(mut self, min: f64, max: f64) -> Self {
        self.min_price = Some(min);
        self.max_price = Some(max);
        self
    }

    pub fn min_rating(mut self, rating: f32) -> Self {
        self.min_rating = Some(rating);
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    pub fn brand(mut self, brand: String) -> Self {
        self.brand = Some(brand);
        self
    }

    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn in_stock_only(mut self) -> Self {
        self.in_stock_only = true;
        self
    }

    fn matches(&self, product: &Product) -> bool {
        if let Some(min_price) = self.min_price {
            if product.price < min_price {
                return false;
            }
        }

        if let Some(max_price) = self.max_price {
            if product.price > max_price {
                return false;
            }
        }

        if let Some(min_rating) = self.min_rating {
            if product.rating < min_rating {
                return false;
            }
        }

        if let Some(ref category) = self.category {
            if product.category != *category {
                return false;
            }
        }

        if let Some(ref brand) = self.brand {
            if product.brand.to_lowercase() != brand.to_lowercase() {
                return false;
            }
        }

        if self.in_stock_only && product.stock == 0 {
            return false;
        }

        if !self.tags.is_empty() {
            let has_any_tag = self.tags.iter().any(|tag| {
                product.tags.iter().any(|product_tag| {
                    product_tag.to_lowercase().contains(&tag.to_lowercase())
                })
            });
            if !has_any_tag {
                return false;
            }
        }

        true
    }
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

    pub fn search_with_filters(&self, query: Option<&str>, filters: &SearchFilters) -> Vec<SearchResult> {
        let mut candidates = HashSet::new();

        if let Some(query_str) = query {
            let name_matches = self.index.search_by_name(query_str);
            candidates.extend(name_matches);

            let tag_results = query_str.split_whitespace()
                .flat_map(|word| self.index.search_by_tag(word))
                .collect::<Vec<_>>();
            candidates.extend(tag_results);
        } else {
            candidates.extend(self.index.all_products().iter().map(|p| p.id));
        }

        if let Some(ref category) = filters.category {
            let category_matches = self.index.search_by_category(category);
            if query.is_some() {
                candidates = candidates.intersection(&category_matches.into_iter().collect()).copied().collect();
            } else {
                candidates.extend(category_matches);
            }
        }

        if let Some(ref brand) = filters.brand {
            let brand_matches = self.index.search_by_brand(brand);
            if query.is_some() || filters.category.is_some() {
                candidates = candidates.intersection(&brand_matches.into_iter().collect()).copied().collect();
            } else {
                candidates.extend(brand_matches);
            }
        }

        let mut results = Vec::new();
        for id in candidates {
            if let Some(product) = self.index.get_product(id) {
                if filters.matches(product) {
                    let score = if let Some(query_str) = query {
                        product.search_score(query_str)
                    } else {
                        product.rating as f64
                    };

                    results.push(SearchResult {
                        product: product.clone(),
                        score,
                        match_type: MatchType::Combined,
                    });
                }
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }

    pub fn search_by_price_range(&self, min_price: f64, max_price: f64) -> Vec<SearchResult> {
        let filters = SearchFilters::new().price_range(min_price, max_price);
        self.search_with_filters(None, &filters)
    }

    pub fn search_by_rating(&self, min_rating: f32) -> Vec<SearchResult> {
        let filters = SearchFilters::new().min_rating(min_rating);
        self.search_with_filters(None, &filters)
    }

    pub fn advanced_search(&self, query: &str, category: Option<Category>, min_price: Option<f64>, max_price: Option<f64>) -> Vec<SearchResult> {
        let mut filters = SearchFilters::new();

        if let Some(cat) = category {
            filters = filters.category(cat);
        }

        if let Some(min) = min_price {
            filters.min_price = Some(min);
        }

        if let Some(max) = max_price {
            filters.max_price = Some(max);
        }

        self.search_with_filters(Some(query), &filters)
    }

    pub fn get_recommendations_for_product(&self, product_id: u64, limit: usize) -> Vec<SearchResult> {
        let recommendations = self.graph.get_recommendations(product_id, limit);
        let mut results = Vec::new();

        for (rec_id, score) in recommendations {
            if let Some(product) = self.index.get_product(rec_id) {
                results.push(SearchResult {
                    product: product.clone(),
                    score: score as f64,
                    match_type: MatchType::Recommendation,
                });
            }
        }

        results
    }

    pub fn search_with_recommendations(&self, query: &str, include_recommendations: bool, limit: usize) -> Vec<SearchResult> {
        let mut all_results = Vec::new();
        let search_results = self.basic_search(query);

        let mut seen_ids = HashSet::new();

        for result in search_results.into_iter().take(limit / 2) {
            seen_ids.insert(result.product.id);
            all_results.push(result);
        }

        if include_recommendations && !all_results.is_empty() {
            let top_product_id = all_results[0].product.id;
            let recommendations = self.graph.get_recommendations(top_product_id, limit);

            for (rec_id, rec_score) in recommendations {
                if !seen_ids.contains(&rec_id) {
                    if let Some(product) = self.index.get_product(rec_id) {
                        seen_ids.insert(rec_id);
                        all_results.push(SearchResult {
                            product: product.clone(),
                            score: rec_score as f64 * 0.8,
                            match_type: MatchType::Recommendation,
                        });
                    }
                }
            }
        }

        all_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        all_results.truncate(limit);
        all_results
    }

    pub fn search_similar_products(&self, product_id: u64) -> Vec<SearchResult> {
        let similar_ids = self.graph.get_similar_products(product_id);
        let mut results = Vec::new();

        for similar_id in similar_ids {
            if let Some(product) = self.index.get_product(similar_id) {
                results.push(SearchResult {
                    product: product.clone(),
                    score: product.rating as f64,
                    match_type: MatchType::Recommendation,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }

    pub fn get_frequently_bought_together(&self, product_id: u64) -> Vec<SearchResult> {
        let bought_together_ids = self.graph.get_frequently_bought_together(product_id);
        let mut results = Vec::new();

        for id in bought_together_ids {
            if let Some(product) = self.index.get_product(id) {
                results.push(SearchResult {
                    product: product.clone(),
                    score: product.rating as f64,
                    match_type: MatchType::Recommendation,
                });
            }
        }

        results
    }

    pub fn hybrid_search(&self, query: Option<&str>, filters: &SearchFilters, use_recommendations: bool) -> Vec<SearchResult> {
        let mut all_results = Vec::new();
        let mut seen_ids = HashSet::new();

        let filtered_results = self.search_with_filters(query, filters);
        let top_scores: Vec<(u64, f64)> = filtered_results.iter()
            .take(3)
            .map(|r| (r.product.id, r.score))
            .collect();

        for result in filtered_results.into_iter() {
            seen_ids.insert(result.product.id);
            all_results.push(result);
        }

        if use_recommendations && !top_scores.is_empty() {
            for (product_id, base_score) in top_scores {
                let recommendations = self.graph.get_recommendations(product_id, 5);

                for (rec_id, rec_score) in recommendations {
                    if !seen_ids.contains(&rec_id) {
                        if let Some(product) = self.index.get_product(rec_id) {
                            if filters.matches(product) {
                                seen_ids.insert(rec_id);
                                all_results.push(SearchResult {
                                    product: product.clone(),
                                    score: base_score * 0.5 + rec_score as f64 * 0.5,
                                    match_type: MatchType::Recommendation,
                                });
                            }
                        }
                    }
                }
            }
        }

        all_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        all_results
    }
}