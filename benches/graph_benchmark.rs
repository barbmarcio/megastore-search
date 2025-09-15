use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use megastore_search::RecommendationGraph;
use megastore_search::graph::RelationType;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn setup_graph_with_products(count: usize) -> RecommendationGraph {
    let mut graph = RecommendationGraph::new();
    let categories = ["Electronics", "Clothing", "Food", "Books", "Sports"];

    for i in 0..count {
        let category = categories[i % categories.len()];
        graph.add_product(i as u64, category.to_string());
    }

    graph
}

fn setup_graph_with_relationships(product_count: usize, edge_density: f32) -> RecommendationGraph {
    let mut graph = setup_graph_with_products(product_count);
    let mut rng = StdRng::seed_from_u64(42);

    let total_possible_edges = (product_count * (product_count - 1)) / 2;
    let target_edges = (total_possible_edges as f32 * edge_density) as usize;

    let relation_types = [
        RelationType::Similar,
        RelationType::BoughtTogether,
        RelationType::SameCategory,
        RelationType::SameBrand,
    ];

    for _ in 0..target_edges {
        let id1 = rng.gen_range(0..product_count as u64);
        let id2 = rng.gen_range(0..product_count as u64);

        if id1 != id2 && !graph.has_edge(id1, id2) {
            let weight = rng.gen_range(0.1..1.0);
            let relation_type = &relation_types[rng.gen_range(0..relation_types.len())];

            graph.add_edge(id1, id2, weight, relation_type.clone());
        }
    }

    graph
}

fn bench_graph_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_construction");

    for size in [100, 500, 1000, 2000].iter() {
        group.bench_with_input(
            BenchmarkId::new("add_products", size),
            size,
            |b, &size| {
                b.iter(|| {
                    setup_graph_with_products(size)
                });
            },
        );
    }

    group.finish();
}

fn bench_edge_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_edges");

    let graph_size = 1000;
    let mut rng = StdRng::seed_from_u64(42);

    group.bench_function("add_single_edge", |b| {
        b.iter_batched(
            || setup_graph_with_products(graph_size),
            |mut graph| {
                let id1 = rng.gen_range(0..graph_size as u64);
                let id2 = rng.gen_range(0..graph_size as u64);
                if id1 != id2 {
                    graph.add_edge(id1, id2, 0.8, RelationType::Similar);
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });

    for density in [0.01, 0.05, 0.1].iter() {
        group.bench_with_input(
            BenchmarkId::new("bulk_edge_addition", format!("{:.0}%", density * 100.0)),
            density,
            |b, &density| {
                b.iter(|| {
                    setup_graph_with_relationships(500, density)
                });
            },
        );
    }

    group.finish();
}

fn bench_recommendation_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_recommendations");

    // Test with different graph sizes and densities
    let configs = [
        (500, 0.02),   // Small, sparse
        (1000, 0.01),  // Medium, sparse
        (500, 0.1),    // Small, dense
        (1000, 0.05),  // Medium, medium
    ];

    for (size, density) in configs.iter() {
        let graph = setup_graph_with_relationships(*size, *density);

        group.bench_with_input(
            BenchmarkId::new("get_recommendations", format!("{}n_{:.0}%d", size, density * 100.0)),
            &graph,
            |b, graph| {
                b.iter(|| {
                    graph.get_recommendations(50, 10)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("get_recommendations_depth_2", format!("{}n_{:.0}%d", size, density * 100.0)),
            &graph,
            |b, graph| {
                b.iter(|| {
                    graph.get_recommendations_depth_2(50, 10)
                });
            },
        );
    }

    group.finish();
}

fn bench_connection_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_queries");

    let graph = setup_graph_with_relationships(1000, 0.05);

    group.bench_function("get_connections", |b| {
        b.iter(|| {
            graph.get_connections(500)
        });
    });

    group.bench_function("has_edge", |b| {
        b.iter(|| {
            graph.has_edge(100, 200)
        });
    });

    group.bench_function("get_similar_products", |b| {
        b.iter(|| {
            graph.get_similar_products(500)
        });
    });

    group.bench_function("get_frequently_bought_together", |b| {
        b.iter(|| {
            graph.get_frequently_bought_together(500)
        });
    });

    group.finish();
}

fn bench_graph_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_scaling");
    group.sample_size(10);

    let sizes = [1000, 5000, 10000];
    let density = 0.01; // Keep density low for larger graphs

    for size in sizes.iter() {
        let graph = setup_graph_with_relationships(*size, density);

        group.bench_with_input(
            BenchmarkId::new("recommendation_scaling", size),
            &graph,
            |b, graph| {
                b.iter(|| {
                    // Simulate getting recommendations for multiple products
                    for i in (0..*size).step_by(*size / 10) {
                        graph.get_recommendations(i as u64, 5);
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_graph_construction,
    bench_edge_operations,
    bench_recommendation_algorithms,
    bench_connection_queries,
    bench_graph_scaling
);
criterion_main!(benches);