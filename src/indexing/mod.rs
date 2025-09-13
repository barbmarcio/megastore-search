use crate::models::{Product, Category};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ProductIndex {
    products: IndexMap<u64, Product>,
    name_index: HashMap<String, HashSet<u64>>,
    brand_index: HashMap<String, HashSet<u64>>,
    category_index: HashMap<Category, HashSet<u64>>,
    tag_index: HashMap<String, HashSet<u64>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        ProductIndex {
            products: IndexMap::new(),
            name_index: HashMap::new(),
            brand_index: HashMap::new(),
            category_index: HashMap::new(),
            tag_index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let id = product.id;

        for word in product.name.split_whitespace() {
            self.name_index
                .entry(word.to_lowercase())
                .or_insert_with(HashSet::new)
                .insert(id);
        }

        self.brand_index
            .entry(product.brand.to_lowercase())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.category_index
            .entry(product.category.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        for tag in &product.tags {
            self.tag_index
                .entry(tag.to_lowercase())
                .or_insert_with(HashSet::new)
                .insert(id);
        }

        self.products.insert(id, product);
    }

    pub fn get_product(&self, id: u64) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn search_by_name(&self, query: &str) -> Vec<u64> {
        let query_lower = query.to_lowercase();
        let mut results = HashSet::new();

        for word in query_lower.split_whitespace() {
            if let Some(ids) = self.name_index.get(word) {
                results.extend(ids);
            }
        }

        results.into_iter().collect()
    }

    pub fn search_by_brand(&self, brand: &str) -> Vec<u64> {
        self.brand_index
            .get(&brand.to_lowercase())
            .map(|ids| ids.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn search_by_category(&self, category: &Category) -> Vec<u64> {
        self.category_index
            .get(category)
            .map(|ids| ids.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<u64> {
        self.tag_index
            .get(&tag.to_lowercase())
            .map(|ids| ids.iter().copied().collect())
            .unwrap_or_default()
    }

    pub fn all_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    pub fn product_count(&self) -> usize {
        self.products.len()
    }

    pub fn remove_product(&mut self, id: u64) -> Option<Product> {
        if let Some(product) = self.products.shift_remove(&id) {
            for word in product.name.split_whitespace() {
                if let Some(ids) = self.name_index.get_mut(&word.to_lowercase()) {
                    ids.remove(&id);
                    if ids.is_empty() {
                        self.name_index.remove(&word.to_lowercase());
                    }
                }
            }

            if let Some(ids) = self.brand_index.get_mut(&product.brand.to_lowercase()) {
                ids.remove(&id);
                if ids.is_empty() {
                    self.brand_index.remove(&product.brand.to_lowercase());
                }
            }

            if let Some(ids) = self.category_index.get_mut(&product.category) {
                ids.remove(&id);
                if ids.is_empty() {
                    self.category_index.remove(&product.category);
                }
            }

            for tag in &product.tags {
                if let Some(ids) = self.tag_index.get_mut(&tag.to_lowercase()) {
                    ids.remove(&id);
                    if ids.is_empty() {
                        self.tag_index.remove(&tag.to_lowercase());
                    }
                }
            }

            Some(product)
        } else {
            None
        }
    }

    pub fn update_product(&mut self, id: u64, product: Product) -> Option<Product> {
        if self.products.contains_key(&id) {
            let old_product = self.remove_product(id);
            self.add_product(product);
            old_product
        } else {
            None
        }
    }
}