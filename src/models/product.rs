use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    Electronics,
    Clothing,
    Food,
    HomeDecor,
    Books,
    Sports,
    Toys,
    Beauty,
    Other(String),
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Electronics => write!(f, "Electronics"),
            Category::Clothing => write!(f, "Clothing"),
            Category::Food => write!(f, "Food"),
            Category::HomeDecor => write!(f, "Home & Decor"),
            Category::Books => write!(f, "Books"),
            Category::Sports => write!(f, "Sports"),
            Category::Toys => write!(f, "Toys"),
            Category::Beauty => write!(f, "Beauty"),
            Category::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub brand: String,
    pub category: Category,
    pub price: f64,
    pub tags: Vec<String>,
    pub rating: f32,
    pub stock: u32,
}

impl Product {
    pub fn new(
        id: u64,
        name: String,
        description: String,
        brand: String,
        category: Category,
        price: f64,
    ) -> Self {
        Product {
            id,
            name,
            description,
            brand,
            category,
            price,
            tags: Vec::new(),
            rating: 0.0,
            stock: 0,
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn search_score(&self, query: &str) -> f64 {
        let query_lower = query.to_lowercase();
        let mut score = 0.0;

        if self.name.to_lowercase().contains(&query_lower) {
            score += 10.0;
        }

        if self.brand.to_lowercase().contains(&query_lower) {
            score += 5.0;
        }

        if self.description.to_lowercase().contains(&query_lower) {
            score += 2.0;
        }

        for tag in &self.tags {
            if tag.to_lowercase().contains(&query_lower) {
                score += 3.0;
            }
        }

        score * (1.0 + self.rating as f64 / 10.0)
    }
}