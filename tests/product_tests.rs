use megastore_search::{Product, Category};

#[test]
fn test_product_creation() {
    let product = Product::new(
        1,
        "Test Product".to_string(),
        "Test Description".to_string(),
        "Test Brand".to_string(),
        Category::Electronics,
        99.99,
    );

    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.description, "Test Description");
    assert_eq!(product.brand, "Test Brand");
    assert_eq!(product.category, Category::Electronics);
    assert_eq!(product.price, 99.99);
    assert_eq!(product.rating, 0.0);
    assert_eq!(product.stock, 0);
    assert!(product.tags.is_empty());
}

#[test]
fn test_add_tag() {
    let mut product = Product::new(
        1,
        "Test".to_string(),
        "Desc".to_string(),
        "Brand".to_string(),
        Category::Electronics,
        50.0,
    );

    product.add_tag("tag1".to_string());
    product.add_tag("tag2".to_string());
    product.add_tag("tag1".to_string()); // Duplicate, should not be added

    assert_eq!(product.tags.len(), 2);
    assert!(product.tags.contains(&"tag1".to_string()));
    assert!(product.tags.contains(&"tag2".to_string()));
}

#[test]
fn test_search_score_exact_name_match() {
    let product = Product::new(
        1,
        "Laptop".to_string(),
        "A great laptop".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        1000.0,
    );

    let score = product.search_score("laptop");
    assert!(score > 0.0);
    assert!(score >= 10.0); // Name match gives 10 points
}

#[test]
fn test_search_score_brand_match() {
    let product = Product::new(
        1,
        "Computer".to_string(),
        "Desktop computer".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        800.0,
    );

    let score = product.search_score("dell");
    assert!(score >= 5.0); // Brand match gives 5 points
}

#[test]
fn test_search_score_with_rating() {
    let mut product = Product::new(
        1,
        "Laptop".to_string(),
        "Description".to_string(),
        "Brand".to_string(),
        Category::Electronics,
        1000.0,
    );
    product.rating = 5.0;

    let score_with_rating = product.search_score("laptop");

    product.rating = 0.0;
    let score_without_rating = product.search_score("laptop");

    assert!(score_with_rating > score_without_rating);
}

#[test]
fn test_category_display() {
    assert_eq!(Category::Electronics.to_string(), "Electronics");
    assert_eq!(Category::Clothing.to_string(), "Clothing");
    assert_eq!(Category::Food.to_string(), "Food");
    assert_eq!(Category::HomeDecor.to_string(), "Home & Decor");
    assert_eq!(Category::Other("Custom".to_string()).to_string(), "Custom");
}