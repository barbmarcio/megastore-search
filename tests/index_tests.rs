use megastore_search::{Product, Category, ProductIndex};

fn create_test_product(id: u64, name: &str, brand: &str, category: Category) -> Product {
    let mut product = Product::new(
        id,
        name.to_string(),
        "Description".to_string(),
        brand.to_string(),
        category,
        100.0,
    );
    product.rating = 4.0;
    product.stock = 10;
    product
}

#[test]
fn test_index_add_and_get_product() {
    let mut index = ProductIndex::new();
    let product = create_test_product(1, "Test Product", "Brand", Category::Electronics);

    index.add_product(product.clone());

    let retrieved = index.get_product(1);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Test Product");
}

#[test]
fn test_search_by_name() {
    let mut index = ProductIndex::new();

    index.add_product(create_test_product(1, "Laptop Dell", "Dell", Category::Electronics));
    index.add_product(create_test_product(2, "Mouse Logitech", "Logitech", Category::Electronics));
    index.add_product(create_test_product(3, "Dell Monitor", "Dell", Category::Electronics));

    let results = index.search_by_name("dell");
    assert_eq!(results.len(), 2);
    assert!(results.contains(&1));
    assert!(results.contains(&3));
}

#[test]
fn test_search_by_brand() {
    let mut index = ProductIndex::new();

    index.add_product(create_test_product(1, "Product 1", "Apple", Category::Electronics));
    index.add_product(create_test_product(2, "Product 2", "Samsung", Category::Electronics));
    index.add_product(create_test_product(3, "Product 3", "Apple", Category::Electronics));

    let results = index.search_by_brand("Apple");
    assert_eq!(results.len(), 2);
    assert!(results.contains(&1));
    assert!(results.contains(&3));
}

#[test]
fn test_search_by_category() {
    let mut index = ProductIndex::new();

    index.add_product(create_test_product(1, "Laptop", "Brand", Category::Electronics));
    index.add_product(create_test_product(2, "Shirt", "Brand", Category::Clothing));
    index.add_product(create_test_product(3, "Phone", "Brand", Category::Electronics));

    let electronics = index.search_by_category(&Category::Electronics);
    assert_eq!(electronics.len(), 2);

    let clothing = index.search_by_category(&Category::Clothing);
    assert_eq!(clothing.len(), 1);
    assert!(clothing.contains(&2));
}

#[test]
fn test_search_by_tag() {
    let mut index = ProductIndex::new();

    let mut product1 = create_test_product(1, "Gaming Laptop", "Asus", Category::Electronics);
    product1.add_tag("gaming".to_string());
    product1.add_tag("laptop".to_string());

    let mut product2 = create_test_product(2, "Office Laptop", "Dell", Category::Electronics);
    product2.add_tag("office".to_string());
    product2.add_tag("laptop".to_string());

    index.add_product(product1);
    index.add_product(product2);

    let gaming_results = index.search_by_tag("gaming");
    assert_eq!(gaming_results.len(), 1);
    assert!(gaming_results.contains(&1));

    let laptop_results = index.search_by_tag("laptop");
    assert_eq!(laptop_results.len(), 2);
}

#[test]
fn test_remove_product() {
    let mut index = ProductIndex::new();
    let product = create_test_product(1, "Test Product", "Brand", Category::Electronics);

    index.add_product(product);
    assert_eq!(index.product_count(), 1);

    let removed = index.remove_product(1);
    assert!(removed.is_some());
    assert_eq!(index.product_count(), 0);
    assert!(index.get_product(1).is_none());
}

#[test]
fn test_update_product() {
    let mut index = ProductIndex::new();
    let original = create_test_product(1, "Original Name", "Brand", Category::Electronics);

    index.add_product(original);

    let updated = create_test_product(1, "Updated Name", "Brand", Category::Electronics);
    let old = index.update_product(1, updated);

    assert!(old.is_some());
    assert_eq!(old.unwrap().name, "Original Name");

    let current = index.get_product(1);
    assert!(current.is_some());
    assert_eq!(current.unwrap().name, "Updated Name");
}

#[test]
fn test_product_count() {
    let mut index = ProductIndex::new();
    assert_eq!(index.product_count(), 0);

    index.add_product(create_test_product(1, "Product 1", "Brand", Category::Electronics));
    assert_eq!(index.product_count(), 1);

    index.add_product(create_test_product(2, "Product 2", "Brand", Category::Electronics));
    assert_eq!(index.product_count(), 2);

    index.remove_product(1);
    assert_eq!(index.product_count(), 1);
}