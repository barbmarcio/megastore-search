use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use megastore_search::{Product, Category, ProductIndex};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn generate_test_products(count: usize) -> Vec<Product> {
    let mut rng = StdRng::seed_from_u64(42);
    let brands = ["Apple", "Samsung", "Dell", "HP", "Asus", "Lenovo", "Sony", "LG"];
    let categories = [Category::Electronics, Category::Clothing, Category::Food, Category::Books];
    let adjectives = ["Pro", "Max", "Ultra", "Gaming", "Business", "Home", "Premium", "Basic"];
    let nouns = ["Laptop", "Phone", "Monitor", "Keyboard", "Mouse", "Tablet", "Watch", "Speaker"];

    (0..count).map(|i| {
        let brand = brands[rng.gen_range(0..brands.len())];
        let adjective = adjectives[rng.gen_range(0..adjectives.len())];
        let noun = nouns[rng.gen_range(0..nouns.len())];

        let mut product = Product::new(
            i as u64,
            format!("{} {} {}", brand, adjective, noun),
            format!("Description for product {}", i),
            brand.to_string(),
            categories[rng.gen_range(0..categories.len())].clone(),
            rng.gen_range(50.0..2000.0),
        );

        product.rating = rng.gen_range(1.0..5.0);
        product.stock = rng.gen_range(0..100);

        // Add some tags
        let tag_count = rng.gen_range(1..4);
        for _ in 0..tag_count {
            let tag = format!("tag{}", rng.gen_range(1..20));
            product.add_tag(tag);
        }

        product
    }).collect()
}

fn bench_add_products(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing_add_products");

    for size in [100, 1000, 5000, 10000].iter() {
        let products = generate_test_products(*size);

        group.bench_with_input(
            BenchmarkId::new("add_products", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut index = ProductIndex::new();
                    for product in &products {
                        index.add_product(product.clone());
                    }
                    index
                });
            },
        );
    }
    group.finish();
}

fn bench_search_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing_search");

    // Setup with 10k products
    let products = generate_test_products(10000);
    let mut index = ProductIndex::new();
    for product in products {
        index.add_product(product);
    }

    group.bench_function("search_by_name", |b| {
        b.iter(|| {
            index.search_by_name("pro")
        });
    });

    group.bench_function("search_by_brand", |b| {
        b.iter(|| {
            index.search_by_brand("apple")
        });
    });

    group.bench_function("search_by_category", |b| {
        b.iter(|| {
            index.search_by_category(&Category::Electronics)
        });
    });

    group.bench_function("search_by_tag", |b| {
        b.iter(|| {
            index.search_by_tag("tag5")
        });
    });

    group.bench_function("get_product", |b| {
        b.iter(|| {
            index.get_product(5000)
        });
    });

    group.finish();
}

fn bench_crud_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing_crud");

    let products = generate_test_products(1000);

    group.bench_function("remove_product", |b| {
        b.iter_batched(
            || {
                let mut index = ProductIndex::new();
                for product in &products {
                    index.add_product(product.clone());
                }
                index
            },
            |mut index| {
                index.remove_product(500)
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("update_product", |b| {
        b.iter_batched(
            || {
                let mut index = ProductIndex::new();
                for product in &products {
                    index.add_product(product.clone());
                }
                index
            },
            |mut index| {
                let updated_product = products[500].clone();
                index.update_product(500, updated_product)
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing_scaling");
    group.sample_size(10); // Reduce sample size for larger datasets

    for size in [1000, 10000, 50000, 100000].iter() {
        let products = generate_test_products(*size);
        let mut index = ProductIndex::new();
        for product in products {
            index.add_product(product);
        }

        group.bench_with_input(
            BenchmarkId::new("search_scaling", size),
            size,
            |b, _| {
                b.iter(|| {
                    let _ = index.search_by_name("gaming");
                    let _ = index.search_by_brand("apple");
                    let _ = index.search_by_category(&Category::Electronics);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_add_products,
    bench_search_operations,
    bench_crud_operations,
    bench_scaling
);
criterion_main!(benches);