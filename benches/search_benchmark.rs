use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use megastore_search::{Product, Category, SearchEngine, SearchFilters};
use megastore_search::graph::RelationType;
use rand::{Rng, SeedableRng, seq::SliceRandom};
use rand::rngs::StdRng;

fn generate_realistic_products(count: usize) -> Vec<Product> {
    let mut rng = StdRng::seed_from_u64(42);

    let brands = [
        "Apple", "Samsung", "Dell", "HP", "Asus", "Lenovo", "Sony", "LG",
        "Nike", "Adidas", "Puma", "Under Armour", "Canon", "Nikon", "Microsoft",
        "Google", "Amazon", "Tesla", "Ford", "Toyota", "BMW", "Mercedes"
    ];

    let electronics_products = [
        "iPhone", "MacBook", "iPad", "Galaxy", "Laptop", "Monitor", "Keyboard",
        "Mouse", "Headphones", "Speaker", "Camera", "Tablet", "Watch", "TV"
    ];

    let clothing_products = [
        "T-Shirt", "Jeans", "Sneakers", "Hoodie", "Jacket", "Shorts", "Dress",
        "Pants", "Shirt", "Sweater", "Cap", "Socks", "Underwear", "Belt"
    ];

    let book_titles = [
        "Programming Rust", "The Art of Computer Programming", "Clean Code",
        "Design Patterns", "Algorithms", "Data Structures", "Machine Learning",
        "Deep Learning", "Web Development", "Mobile Development"
    ];

    let categories = [
        (Category::Electronics, &electronics_products[..], 50.0..3000.0),
        (Category::Clothing, &clothing_products[..], 20.0..300.0),
        (Category::Books, &book_titles[..], 10.0..100.0),
    ];

    (0..count).map(|i| {
        let (category, products, price_range) = &categories[i % categories.len()];
        let brand = brands[rng.gen_range(0..brands.len())];
        let product_name = products[rng.gen_range(0..products.len())];

        let mut product = Product::new(
            i as u64,
            format!("{} {}", brand, product_name),
            format!("High-quality {} from {}", product_name.to_lowercase(), brand),
            brand.to_string(),
            category.clone(),
            rng.gen_range(price_range.clone()),
        );

        product.rating = rng.gen_range(3.0..5.0);
        product.stock = rng.gen_range(0..200);

        // Add relevant tags
        let tag_pools = match category {
            Category::Electronics => vec!["tech", "digital", "wireless", "smart", "premium", "gaming"],
            Category::Clothing => vec!["fashion", "comfortable", "stylish", "sport", "casual", "premium"],
            Category::Books => vec!["education", "technical", "programming", "reference", "bestseller"],
            _ => vec!["quality", "popular", "recommended"],
        };

        let num_tags = rng.gen_range(2..5);
        let selected_tags: Vec<_> = tag_pools.choose_multiple(&mut rng, num_tags).collect();
        for &tag in selected_tags {
            product.add_tag(tag.to_string());
        }

        product
    }).collect()
}

fn setup_search_engine(product_count: usize, with_relationships: bool) -> SearchEngine {
    let mut engine = SearchEngine::new();
    let products = generate_realistic_products(product_count);
    let mut rng = StdRng::seed_from_u64(42);

    for product in products {
        engine.add_product(product);
    }

    if with_relationships {
        // Add realistic relationships
        let num_relationships = product_count / 5; // 20% relationship density

        for _ in 0..num_relationships {
            let id1 = rng.gen_range(0..product_count as u64);
            let id2 = rng.gen_range(0..product_count as u64);

            if id1 != id2 {
                let weight = rng.gen_range(0.3..0.9);
                let relation_types = [
                    RelationType::Similar,
                    RelationType::BoughtTogether,
                    RelationType::SameCategory,
                ];
                let relation = &relation_types[rng.gen_range(0..relation_types.len())];

                engine.add_product_relation(id1, id2, weight, relation.clone());
            }
        }
    }

    engine
}

fn bench_basic_search_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_basic");

    let engine = setup_search_engine(10000, false);

    let search_terms = ["laptop", "phone", "nike", "apple", "gaming", "premium"];

    group.bench_function("basic_search", |b| {
        b.iter(|| {
            let term = search_terms[rand::thread_rng().gen_range(0..search_terms.len())];
            engine.basic_search(term)
        });
    });

    group.bench_function("search_by_category", |b| {
        b.iter(|| {
            engine.search_by_category(&Category::Electronics)
        });
    });

    group.bench_function("search_by_brand", |b| {
        b.iter(|| {
            engine.search_by_brand("Apple")
        });
    });

    group.bench_function("search_by_price_range", |b| {
        b.iter(|| {
            engine.search_by_price_range(100.0, 500.0)
        });
    });

    group.bench_function("search_by_rating", |b| {
        b.iter(|| {
            engine.search_by_rating(4.0)
        });
    });

    group.finish();
}

fn bench_advanced_search_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_advanced");

    let engine = setup_search_engine(10000, false);

    group.bench_function("advanced_search", |b| {
        b.iter(|| {
            engine.advanced_search(
                "gaming",
                Some(Category::Electronics),
                Some(100.0),
                Some(1000.0)
            )
        });
    });

    let filter_configs = [
        ("simple_filter", SearchFilters::new().min_rating(4.0)),
        ("price_filter", SearchFilters::new().price_range(50.0, 500.0)),
        ("category_filter", SearchFilters::new().category(Category::Electronics)),
        ("complex_filter", SearchFilters::new()
            .category(Category::Electronics)
            .min_rating(4.0)
            .price_range(100.0, 1000.0)
            .add_tag("tech".to_string())
            .in_stock_only()),
    ];

    for (name, filters) in filter_configs.iter() {
        group.bench_function(*name, |b| {
            b.iter(|| {
                engine.search_with_filters(Some("apple"), filters)
            });
        });
    }

    group.finish();
}

fn bench_recommendation_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_recommendations");

    let engine = setup_search_engine(5000, true);

    group.bench_function("search_with_recommendations", |b| {
        b.iter(|| {
            engine.search_with_recommendations("gaming laptop", true, 20)
        });
    });

    group.bench_function("get_recommendations_for_product", |b| {
        b.iter(|| {
            engine.get_recommendations_for_product(100, 10)
        });
    });

    group.bench_function("search_similar_products", |b| {
        b.iter(|| {
            engine.search_similar_products(500)
        });
    });

    group.bench_function("get_frequently_bought_together", |b| {
        b.iter(|| {
            engine.get_frequently_bought_together(300)
        });
    });

    group.bench_function("hybrid_search", |b| {
        b.iter(|| {
            let filters = SearchFilters::new()
                .category(Category::Electronics)
                .min_rating(3.5);
            engine.hybrid_search(Some("premium"), &filters, true)
        });
    });

    group.finish();
}

fn bench_scaling_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_scaling");
    group.sample_size(10);

    let sizes = [1000, 5000, 10000, 50000];

    for size in sizes.iter() {
        let engine = setup_search_engine(*size, false);

        group.bench_with_input(
            BenchmarkId::new("search_scaling_basic", size),
            &engine,
            |b, engine| {
                b.iter(|| {
                    engine.basic_search("laptop")
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("search_scaling_filtered", size),
            &engine,
            |b, engine| {
                b.iter(|| {
                    let filters = SearchFilters::new()
                        .category(Category::Electronics)
                        .price_range(100.0, 1000.0);
                    engine.search_with_filters(Some("gaming"), &filters)
                });
            },
        );
    }

    group.finish();
}

fn bench_concurrent_search_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_concurrent");

    let engine = setup_search_engine(10000, true);

    group.bench_function("mixed_workload", |b| {
        b.iter(|| {
            // Simulate different types of searches happening concurrently
            let _ = engine.basic_search("laptop");
            let _ = engine.search_by_category(&Category::Clothing);
            let _ = engine.search_by_price_range(50.0, 200.0);
            let _ = engine.get_recommendations_for_product(100, 5);

            let filters = SearchFilters::new()
                .min_rating(4.0)
                .add_tag("premium".to_string());
            let _ = engine.search_with_filters(Some("apple"), &filters);
        });
    });

    group.finish();
}

fn bench_memory_usage_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_memory");

    group.bench_function("large_result_set", |b| {
        let engine = setup_search_engine(50000, false);

        b.iter(|| {
            // Search that returns many results
            let results = engine.search_by_category(&Category::Electronics);
            results.len() // Force evaluation
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_search_operations,
    bench_advanced_search_operations,
    bench_recommendation_integration,
    bench_scaling_performance,
    bench_concurrent_search_simulation,
    bench_memory_usage_simulation
);
criterion_main!(benches);