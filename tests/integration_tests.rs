use megastore_search::{Product, Category, SearchEngine, SearchFilters};
use megastore_search::graph::RelationType;

fn setup_test_catalog() -> SearchEngine {
    let mut engine = SearchEngine::new();

    // Gaming laptops
    let mut laptop1 = Product::new(
        1,
        "ASUS ROG Gaming Laptop".to_string(),
        "High-performance gaming laptop with RTX 4060".to_string(),
        "ASUS".to_string(),
        Category::Electronics,
        1200.0,
    );
    laptop1.add_tag("gaming".to_string());
    laptop1.add_tag("laptop".to_string());
    laptop1.add_tag("rtx".to_string());
    laptop1.rating = 4.5;
    laptop1.stock = 5;

    let mut laptop2 = Product::new(
        2,
        "Dell Gaming Laptop G15".to_string(),
        "Affordable gaming laptop for students".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        900.0,
    );
    laptop2.add_tag("gaming".to_string());
    laptop2.add_tag("laptop".to_string());
    laptop2.add_tag("budget".to_string());
    laptop2.rating = 4.2;
    laptop2.stock = 8;

    // Gaming peripherals
    let mut mouse = Product::new(
        3,
        "Logitech G Pro Gaming Mouse".to_string(),
        "Professional gaming mouse with RGB".to_string(),
        "Logitech".to_string(),
        Category::Electronics,
        80.0,
    );
    mouse.add_tag("gaming".to_string());
    mouse.add_tag("mouse".to_string());
    mouse.add_tag("rgb".to_string());
    mouse.rating = 4.7;
    mouse.stock = 20;

    let mut keyboard = Product::new(
        4,
        "Corsair K95 Gaming Keyboard".to_string(),
        "Mechanical gaming keyboard with Cherry MX switches".to_string(),
        "Corsair".to_string(),
        Category::Electronics,
        150.0,
    );
    keyboard.add_tag("gaming".to_string());
    keyboard.add_tag("keyboard".to_string());
    keyboard.add_tag("mechanical".to_string());
    keyboard.rating = 4.6;
    keyboard.stock = 12;

    // Office equipment
    let mut office_laptop = Product::new(
        5,
        "Dell Inspiron Office Laptop".to_string(),
        "Reliable laptop for office work".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        600.0,
    );
    office_laptop.add_tag("office".to_string());
    office_laptop.add_tag("laptop".to_string());
    office_laptop.add_tag("business".to_string());
    office_laptop.rating = 4.0;
    office_laptop.stock = 15;

    // Add products to engine
    engine.add_product(laptop1);
    engine.add_product(laptop2);
    engine.add_product(mouse);
    engine.add_product(keyboard);
    engine.add_product(office_laptop);

    // Set up relationships
    engine.add_product_relation(1, 3, 0.9, RelationType::BoughtTogether); // Gaming laptop + mouse
    engine.add_product_relation(1, 4, 0.8, RelationType::BoughtTogether); // Gaming laptop + keyboard
    engine.add_product_relation(3, 4, 0.7, RelationType::BoughtTogether); // Mouse + keyboard
    engine.add_product_relation(1, 2, 0.8, RelationType::Similar); // Similar gaming laptops
    engine.add_product_relation(2, 5, 0.3, RelationType::SameBrand); // Dell laptops

    engine
}

#[test]
fn test_full_catalog_search() {
    let engine = setup_test_catalog();

    // Test basic search
    let gaming_results = engine.basic_search("gaming");
    assert_eq!(gaming_results.len(), 4);

    // Test search by category
    let electronics = engine.search_by_category(&Category::Electronics);
    assert_eq!(electronics.len(), 5);

    // Test brand search
    let dell_products = engine.search_by_brand("Dell");
    assert_eq!(dell_products.len(), 2);
}

#[test]
fn test_price_filtering() {
    let engine = setup_test_catalog();

    // Budget range
    let budget_results = engine.search_by_price_range(50.0, 200.0);
    assert_eq!(budget_results.len(), 2); // Mouse and keyboard

    // Mid-range
    let mid_range = engine.search_by_price_range(500.0, 1000.0);
    assert_eq!(mid_range.len(), 2); // Dell gaming and office laptops

    // High-end
    let high_end = engine.search_by_price_range(1000.0, 2000.0);
    assert_eq!(high_end.len(), 1); // ASUS gaming laptop
}

#[test]
fn test_rating_filtering() {
    let engine = setup_test_catalog();

    let high_rated = engine.search_by_rating(4.5);
    assert_eq!(high_rated.len(), 3); // ASUS laptop, mouse, keyboard

    let medium_rated = engine.search_by_rating(4.0);
    assert_eq!(medium_rated.len(), 5); // All products
}

#[test]
fn test_advanced_search_scenarios() {
    let engine = setup_test_catalog();

    // Gaming laptops under $1000
    let budget_gaming = engine.advanced_search(
        "gaming laptop",
        Some(Category::Electronics),
        None,
        Some(1000.0)
    );
    assert_eq!(budget_gaming.len(), 1);
    assert_eq!(budget_gaming[0].product.name, "Dell Gaming Laptop G15");

    // High-end gaming equipment
    let high_end_gaming = engine.advanced_search(
        "gaming",
        Some(Category::Electronics),
        Some(1000.0),
        None
    );
    assert_eq!(high_end_gaming.len(), 1);
    assert!(high_end_gaming[0].product.name.contains("ASUS ROG"));
}

#[test]
fn test_complex_filtering() {
    let engine = setup_test_catalog();

    // Gaming products with high rating and reasonable price
    let filters = SearchFilters::new()
        .add_tag("gaming".to_string())
        .min_rating(4.5)
        .price_range(50.0, 200.0);

    let filtered_results = engine.search_with_filters(None, &filters);
    assert_eq!(filtered_results.len(), 2); // Mouse and keyboard
}

#[test]
fn test_recommendation_system() {
    let engine = setup_test_catalog();

    // Recommendations for gaming laptop
    let laptop_recommendations = engine.get_recommendations_for_product(1, 5);
    assert!(!laptop_recommendations.is_empty());

    // Should recommend gaming peripherals
    let recommended_names: Vec<String> = laptop_recommendations
        .iter()
        .map(|r| r.product.name.clone())
        .collect();
    assert!(recommended_names.iter().any(|name| name.contains("Mouse")));
    assert!(recommended_names.iter().any(|name| name.contains("Keyboard")));
}

#[test]
fn test_search_with_recommendations_integration() {
    let engine = setup_test_catalog();

    let results_with_rec = engine.search_with_recommendations("gaming laptop", true, 10);
    let results_without_rec = engine.search_with_recommendations("gaming laptop", false, 10);

    // With recommendations should include more results
    assert!(results_with_rec.len() > results_without_rec.len());

    // Should include both laptops and recommended peripherals
    let product_names: Vec<String> = results_with_rec
        .iter()
        .map(|r| r.product.name.clone())
        .collect();

    assert!(product_names.iter().any(|name| name.contains("Laptop")));
    assert!(product_names.iter().any(|name| name.contains("Mouse") || name.contains("Keyboard")));
}

#[test]
fn test_hybrid_search_complete() {
    let engine = setup_test_catalog();

    let filters = SearchFilters::new()
        .category(Category::Electronics)
        .min_rating(4.0)
        .price_range(50.0, 1500.0);

    let hybrid_results = engine.hybrid_search(Some("gaming"), &filters, true);

    // Should include both direct search results and recommendations
    assert!(!hybrid_results.is_empty());

    // Verify mixed match types
    let has_search_results = hybrid_results.iter().any(|r| {
        matches!(r.match_type, megastore_search::search::MatchType::Combined)
    });
    let has_recommendations = hybrid_results.iter().any(|r| {
        matches!(r.match_type, megastore_search::search::MatchType::Recommendation)
    });

    assert!(has_search_results || has_recommendations);
}

#[test]
fn test_similar_products_workflow() {
    let engine = setup_test_catalog();

    // Find similar products to ASUS gaming laptop
    let similar = engine.search_similar_products(1);
    assert_eq!(similar.len(), 1);
    assert!(similar[0].product.name.contains("Dell Gaming"));
}

#[test]
fn test_frequently_bought_together() {
    let engine = setup_test_catalog();

    // Products bought with gaming laptop
    let bought_together = engine.get_frequently_bought_together(1);
    assert_eq!(bought_together.len(), 2); // Mouse and keyboard

    let product_names: Vec<String> = bought_together
        .iter()
        .map(|r| r.product.name.clone())
        .collect();

    assert!(product_names.iter().any(|name| name.contains("Mouse")));
    assert!(product_names.iter().any(|name| name.contains("Keyboard")));
}

#[test]
fn test_stock_filtering() {
    let engine = setup_test_catalog();

    let in_stock_filters = SearchFilters::new()
        .category(Category::Electronics)
        .in_stock_only();

    let in_stock_results = engine.search_with_filters(None, &in_stock_filters);
    assert_eq!(in_stock_results.len(), 5); // All products have stock > 0

    // Test would be more meaningful with some out-of-stock products
}

#[test]
fn test_performance_with_larger_dataset() {
    let mut engine = SearchEngine::new();

    // Add many products to test performance
    for i in 1..=100 {
        let mut product = Product::new(
            i,
            format!("Product {}", i),
            format!("Description for product {}", i),
            if i % 3 == 0 { "Apple" } else if i % 3 == 1 { "Samsung" } else { "Google" }.to_string(),
            if i % 2 == 0 { Category::Electronics } else { Category::Clothing },
            (i as f64) * 10.0,
        );
        product.rating = 3.0 + (i % 3) as f32;
        product.stock = i as u32;
        engine.add_product(product);
    }

    // Add some relationships
    for i in 1..=50 {
        if i + 1 <= 100 {
            engine.add_product_relation(i, i + 1, 0.5, RelationType::Similar);
        }
    }

    // Test searches still work with larger dataset
    let results = engine.basic_search("Product");
    assert_eq!(results.len(), 100);

    let apple_products = engine.search_by_brand("Apple");
    assert_eq!(apple_products.len(), 33); // Every 3rd product

    let electronics = engine.search_by_category(&Category::Electronics);
    assert_eq!(electronics.len(), 50); // Every even product
}