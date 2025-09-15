use megastore_search::{RecommendationGraph};
use megastore_search::graph::RelationType;

#[test]
fn test_add_product_to_graph() {
    let mut graph = RecommendationGraph::new();

    let _node1 = graph.add_product(1, "Electronics".to_string());
    let _node2 = graph.add_product(2, "Electronics".to_string());

    assert_eq!(graph.product_count(), 2);
    assert!(graph.get_node_index(1).is_some());
    assert!(graph.get_node_index(2).is_some());
}

#[test]
fn test_add_duplicate_product() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(1, "Electronics".to_string()); // Duplicate

    assert_eq!(graph.product_count(), 1);
}

#[test]
fn test_add_edge() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());

    let success = graph.add_edge(1, 2, 0.8, RelationType::Similar);
    assert!(success);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_add_edge_invalid_nodes() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());

    let success = graph.add_edge(1, 999, 0.8, RelationType::Similar);
    assert!(!success);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_connect_similar_products() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());

    graph.connect_similar_products(1, 2, 0.9);

    let connections = graph.get_connections(1);
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].0, 2);
    assert_eq!(connections[0].2, RelationType::Similar);
}

#[test]
fn test_connect_bought_together() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());

    graph.connect_bought_together(1, 2, 0.75);

    let connections = graph.get_connections(1);
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].2, RelationType::BoughtTogether);
}

#[test]
fn test_get_connections() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Clothing".to_string());

    graph.connect_similar_products(1, 2, 0.8);
    graph.connect_bought_together(1, 3, 0.6);

    let connections = graph.get_connections(1);
    assert_eq!(connections.len(), 2);
}

#[test]
fn test_has_edge() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Electronics".to_string());

    graph.connect_similar_products(1, 2, 0.8);

    assert!(graph.has_edge(1, 2));
    assert!(graph.has_edge(2, 1)); // Undirected graph
    assert!(!graph.has_edge(1, 3));
}

#[test]
fn test_get_recommendations() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Electronics".to_string());
    graph.add_product(4, "Electronics".to_string());

    graph.connect_bought_together(1, 2, 0.9); // Highest score (0.9 * 1.5 = 1.35)
    graph.connect_similar_products(1, 3, 0.8); // Medium score (0.8 * 1.3 = 1.04)
    graph.connect_same_category(1, 4); // Lowest score (0.5 * 1.0 = 0.5)

    let recommendations = graph.get_recommendations(1, 3);
    assert_eq!(recommendations.len(), 3);

    // Check ordering (highest score first)
    assert!(recommendations[0].1 > recommendations[1].1);
    assert!(recommendations[1].1 > recommendations[2].1);
}

#[test]
fn test_get_similar_products() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Electronics".to_string());

    graph.connect_similar_products(1, 2, 0.9);
    graph.connect_bought_together(1, 3, 0.7);

    let similar = graph.get_similar_products(1);
    assert_eq!(similar.len(), 1);
    assert_eq!(similar[0], 2);
}

#[test]
fn test_get_frequently_bought_together() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Electronics".to_string());

    graph.connect_bought_together(1, 2, 0.8);
    graph.connect_similar_products(1, 3, 0.7);

    let bought_together = graph.get_frequently_bought_together(1);
    assert_eq!(bought_together.len(), 1);
    assert_eq!(bought_together[0], 2);
}

#[test]
fn test_recommendations_depth_2() {
    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Electronics".to_string());
    graph.add_product(4, "Electronics".to_string());

    // 1 -> 2 -> 3
    graph.connect_similar_products(1, 2, 0.8);
    graph.connect_similar_products(2, 3, 0.7);
    // 1 -> 4 (direct)
    graph.connect_bought_together(1, 4, 0.6);

    let recommendations = graph.get_recommendations_depth_2(1, 5);

    // Should include direct connections (2, 4) and indirect (3)
    let product_ids: Vec<u64> = recommendations.iter().map(|(id, _)| *id).collect();
    assert!(product_ids.contains(&2));
    assert!(product_ids.contains(&3)); // Second-degree connection
    assert!(product_ids.contains(&4));
}