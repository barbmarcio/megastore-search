use megastore_search::{Product, Category, SearchEngine, SearchFilters};
use megastore_search::search::MatchType;
use megastore_search::graph::RelationType;

fn create_test_product(id: u64, name: &str, brand: &str, category: Category, price: f64, rating: f32) -> Product {
    let mut product = Product::new(
        id,
        name.to_string(),
        "Description".to_string(),
        brand.to_string(),
        category,
        price,
    );
    product.rating = rating;
    product.stock = 10;
    product
}

#[test]
fn test_search_engine_add_product() {
    let mut engine = SearchEngine::new();
    let product = create_test_product(1, "Test Product", "Brand", Category::Electronics, 100.0, 4.0);

    engine.add_product(product);

    assert_eq!(engine.get_product_count(), 1);
    let (nodes, _) = engine.get_graph_stats();
    assert_eq!(nodes, 1);
}

#[test]
fn test_basic_search() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Gaming Laptop", "Asus", Category::Electronics, 1200.0, 4.5));
    engine.add_product(create_test_product(2, "Office Mouse", "Logitech", Category::Electronics, 25.0, 4.0));
    engine.add_product(create_test_product(3, "Gaming Mouse", "Logitech", Category::Electronics, 60.0, 4.8));

    let results = engine.basic_search("gaming");
    assert_eq!(results.len(), 2);

    // Check that results are sorted by score
    assert!(results[0].score >= results[1].score);
}

#[test]
fn test_search_by_category() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Laptop", "Dell", Category::Electronics, 1000.0, 4.5));
    engine.add_product(create_test_product(2, "T-Shirt", "Nike", Category::Clothing, 30.0, 4.0));
    engine.add_product(create_test_product(3, "Phone", "Apple", Category::Electronics, 800.0, 4.7));

    let electronics = engine.search_by_category(&Category::Electronics);
    assert_eq!(electronics.len(), 2);

    let clothing = engine.search_by_category(&Category::Clothing);
    assert_eq!(clothing.len(), 1);
    assert_eq!(clothing[0].product.name, "T-Shirt");
}

#[test]
fn test_search_by_brand() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "iPhone", "Apple", Category::Electronics, 900.0, 4.6));
    engine.add_product(create_test_product(2, "MacBook", "Apple", Category::Electronics, 1500.0, 4.7));
    engine.add_product(create_test_product(3, "Galaxy", "Samsung", Category::Electronics, 800.0, 4.4));

    let apple_products = engine.search_by_brand("Apple");
    assert_eq!(apple_products.len(), 2);
}

#[test]
fn test_search_filters() {
    let filters = SearchFilters::new()
        .price_range(100.0, 500.0)
        .min_rating(4.0)
        .category(Category::Electronics);

    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Cheap Phone", "Brand", Category::Electronics, 50.0, 3.0)); // Below price and rating
    engine.add_product(create_test_product(2, "Good Phone", "Brand", Category::Electronics, 300.0, 4.5)); // Matches all
    engine.add_product(create_test_product(3, "Expensive Phone", "Brand", Category::Electronics, 1000.0, 4.8)); // Above price
    engine.add_product(create_test_product(4, "Shirt", "Brand", Category::Clothing, 200.0, 4.2)); // Wrong category

    let results = engine.search_with_filters(None, &filters);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].product.name, "Good Phone");
}

#[test]
fn test_search_by_price_range() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Cheap", "Brand", Category::Electronics, 50.0, 4.0));
    engine.add_product(create_test_product(2, "Medium", "Brand", Category::Electronics, 150.0, 4.0));
    engine.add_product(create_test_product(3, "Expensive", "Brand", Category::Electronics, 500.0, 4.0));

    let results = engine.search_by_price_range(100.0, 200.0);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].product.name, "Medium");
}

#[test]
fn test_search_by_rating() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Low Rated", "Brand", Category::Electronics, 100.0, 3.0));
    engine.add_product(create_test_product(2, "High Rated", "Brand", Category::Electronics, 100.0, 4.5));
    engine.add_product(create_test_product(3, "Very High Rated", "Brand", Category::Electronics, 100.0, 4.8));

    let results = engine.search_by_rating(4.0);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_advanced_search() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Gaming Laptop", "Asus", Category::Electronics, 1200.0, 4.5));
    engine.add_product(create_test_product(2, "Office Laptop", "Dell", Category::Electronics, 800.0, 4.0));
    engine.add_product(create_test_product(3, "Gaming Desktop", "Asus", Category::Electronics, 1500.0, 4.7));

    let results = engine.advanced_search(
        "laptop",
        Some(Category::Electronics),
        Some(700.0),
        Some(1300.0)
    );

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.product.name.contains("Laptop")));
    assert!(results.iter().all(|r| r.product.price >= 700.0 && r.product.price <= 1300.0));
}

#[test]
fn test_add_product_relation() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Laptop", "Dell", Category::Electronics, 1000.0, 4.5));
    engine.add_product(create_test_product(2, "Mouse", "Logitech", Category::Electronics, 50.0, 4.0));

    engine.add_product_relation(1, 2, 0.8, RelationType::BoughtTogether);

    let (_, edges) = engine.get_graph_stats();
    assert_eq!(edges, 1);
}

#[test]
fn test_get_recommendations_for_product() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Laptop", "Dell", Category::Electronics, 1000.0, 4.5));
    engine.add_product(create_test_product(2, "Mouse", "Logitech", Category::Electronics, 50.0, 4.0));
    engine.add_product(create_test_product(3, "Keyboard", "Logitech", Category::Electronics, 80.0, 4.2));

    engine.add_product_relation(1, 2, 0.8, RelationType::BoughtTogether);
    engine.add_product_relation(1, 3, 0.6, RelationType::Similar);

    let recommendations = engine.get_recommendations_for_product(1, 5);
    assert_eq!(recommendations.len(), 2);
    assert!(recommendations.iter().all(|r| matches!(r.match_type, MatchType::Recommendation)));
}

#[test]
fn test_search_with_recommendations() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Gaming Laptop", "Asus", Category::Electronics, 1200.0, 4.5));
    engine.add_product(create_test_product(2, "Gaming Mouse", "Logitech", Category::Electronics, 60.0, 4.3));
    engine.add_product(create_test_product(3, "Office Laptop", "Dell", Category::Electronics, 800.0, 4.0));

    engine.add_product_relation(1, 2, 0.9, RelationType::BoughtTogether);

    let results_with_rec = engine.search_with_recommendations("laptop", true, 5);
    let results_without_rec = engine.search_with_recommendations("laptop", false, 5);

    assert!(results_with_rec.len() >= results_without_rec.len());
}

#[test]
fn test_search_similar_products() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Laptop A", "Brand", Category::Electronics, 1000.0, 4.5));
    engine.add_product(create_test_product(2, "Laptop B", "Brand", Category::Electronics, 1100.0, 4.3));
    engine.add_product(create_test_product(3, "Mouse", "Brand", Category::Electronics, 50.0, 4.0));

    engine.add_product_relation(1, 2, 0.8, RelationType::Similar);
    engine.add_product_relation(1, 3, 0.6, RelationType::BoughtTogether);

    let similar = engine.search_similar_products(1);
    assert_eq!(similar.len(), 1);
    assert_eq!(similar[0].product.name, "Laptop B");
}

#[test]
fn test_hybrid_search() {
    let mut engine = SearchEngine::new();

    engine.add_product(create_test_product(1, "Gaming Laptop", "Asus", Category::Electronics, 1200.0, 4.5));
    engine.add_product(create_test_product(2, "Gaming Mouse", "Logitech", Category::Electronics, 60.0, 4.3));
    engine.add_product(create_test_product(3, "Office Laptop", "Dell", Category::Electronics, 800.0, 4.0));

    engine.add_product_relation(1, 2, 0.9, RelationType::BoughtTogether);

    let filters = SearchFilters::new().category(Category::Electronics);

    let hybrid_results = engine.hybrid_search(Some("gaming"), &filters, true);
    let regular_results = engine.hybrid_search(Some("gaming"), &filters, false);

    assert!(hybrid_results.len() >= regular_results.len());
}