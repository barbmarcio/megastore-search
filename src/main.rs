use megastore_search::{Product, Category, ProductIndex, RecommendationGraph, SearchEngine};

fn main() {
    println!("🛍️ MegaStore Search System");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Sistema de busca otimizado com indexação HashMap\n");

    let mut index = ProductIndex::new();

    let mut product1 = Product::new(
        1,
        "Notebook Dell Inspiron".to_string(),
        "Notebook para trabalho e estudos".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        3500.0,
    );
    product1.add_tag("laptop".to_string());
    product1.add_tag("computador".to_string());
    product1.rating = 4.5;
    product1.stock = 10;

    let mut product2 = Product::new(
        2,
        "Mouse Gamer Logitech".to_string(),
        "Mouse com RGB e alta precisão".to_string(),
        "Logitech".to_string(),
        Category::Electronics,
        250.0,
    );
    product2.add_tag("gaming".to_string());
    product2.add_tag("periférico".to_string());
    product2.rating = 4.8;
    product2.stock = 50;

    let mut product3 = Product::new(
        3,
        "Camiseta Nike Dri-Fit".to_string(),
        "Camiseta esportiva com tecnologia Dri-Fit".to_string(),
        "Nike".to_string(),
        Category::Clothing,
        120.0,
    );
    product3.add_tag("esporte".to_string());
    product3.add_tag("fitness".to_string());
    product3.rating = 4.3;
    product3.stock = 100;

    let mut product4 = Product::new(
        4,
        "Notebook Asus Vivobook".to_string(),
        "Notebook fino e leve para o dia a dia".to_string(),
        "Asus".to_string(),
        Category::Electronics,
        2800.0,
    );
    product4.add_tag("laptop".to_string());
    product4.add_tag("portátil".to_string());
    product4.rating = 4.2;
    product4.stock = 15;

    index.add_product(product1);
    index.add_product(product2);
    index.add_product(product3);
    index.add_product(product4);

    println!("📦 Total de produtos indexados: {}\n", index.product_count());

    println!("🔍 Testando buscas por nome:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let search_results = index.search_by_name("notebook");
    println!("Busca por 'notebook': {} resultados", search_results.len());
    for id in &search_results {
        if let Some(product) = index.get_product(*id) {
            println!("  → {} (R$ {:.2})", product.name, product.price);
        }
    }

    println!("\n🏷️ Testando buscas por marca:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let dell_products = index.search_by_brand("Dell");
    println!("Produtos Dell: {} resultados", dell_products.len());
    for id in &dell_products {
        if let Some(product) = index.get_product(*id) {
            println!("  → {} (⭐ {:.1})", product.name, product.rating);
        }
    }

    println!("\n📂 Testando buscas por categoria:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let electronics = index.search_by_category(&Category::Electronics);
    println!("Eletrônicos: {} produtos", electronics.len());
    for id in &electronics {
        if let Some(product) = index.get_product(*id) {
            println!("  → {} - {} (📦 {} unidades)", product.name, product.brand, product.stock);
        }
    }

    println!("\n🏷️ Testando buscas por tag:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let laptop_products = index.search_by_tag("laptop");
    println!("Produtos com tag 'laptop': {} resultados", laptop_products.len());
    for id in &laptop_products {
        if let Some(product) = index.get_product(*id) {
            println!("  → {} - R$ {:.2}", product.name, product.price);
        }
    }

    println!("\n✅ Sistema de indexação funcionando com sucesso!");
    println!("Complexidade de busca: O(1) para acesso direto ao índice");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Testando Grafo de Recomendações");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut graph = RecommendationGraph::new();

    graph.add_product(1, "Electronics".to_string());
    graph.add_product(2, "Electronics".to_string());
    graph.add_product(3, "Clothing".to_string());
    graph.add_product(4, "Electronics".to_string());

    println!("📊 Grafo inicial:");
    println!("  • Nós (produtos): {}", graph.product_count());
    println!("  • Arestas (relações): {}", graph.edge_count());

    println!("\n🔗 Criando relações entre produtos:");

    graph.connect_similar_products(1, 4, 0.85);
    println!("  ✓ Notebooks Dell e Asus são similares (85% similaridade)");

    graph.connect_bought_together(1, 2, 0.75);
    println!("  ✓ Notebook Dell e Mouse frequentemente comprados juntos (75%)");

    graph.connect_same_category(2, 4);
    println!("  ✓ Mouse e Notebook Asus na mesma categoria");

    graph.connect_same_brand(1, 2);
    println!("  ✓ Produtos 1 e 2 conectados por marca");

    println!("\n📊 Grafo atualizado:");
    println!("  • Nós (produtos): {}", graph.product_count());
    println!("  • Arestas (relações): {}", graph.edge_count());

    println!("\n🔍 Conexões do produto 1 (Notebook Dell):");
    let connections = graph.get_connections(1);
    for (product_id, weight, relation_type) in connections {
        let relation_str = match relation_type {
            megastore_search::graph::RelationType::Similar => "Similar",
            megastore_search::graph::RelationType::BoughtTogether => "Comprado junto",
            megastore_search::graph::RelationType::SameCategory => "Mesma categoria",
            megastore_search::graph::RelationType::SameBrand => "Mesma marca",
        };
        println!("  → Produto {} | Peso: {:.2} | Tipo: {}", product_id, weight, relation_str);
    }

    println!("\n✅ Sistema de relações no grafo funcionando!");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 Algoritmo de Recomendação");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n📌 Recomendações para Produto 1 (Notebook Dell):");
    let recommendations = graph.get_recommendations(1, 5);
    for (product_id, score) in &recommendations {
        if let Some(product) = index.get_product(*product_id) {
            println!("  → {} | Score: {:.2}", product.name, score);
        }
    }

    println!("\n📌 Produtos similares ao Produto 1:");
    let similar = graph.get_similar_products(1);
    for id in similar {
        if let Some(product) = index.get_product(id) {
            println!("  → {}", product.name);
        }
    }

    println!("\n📌 Frequentemente comprados com Produto 1:");
    let bought_together = graph.get_frequently_bought_together(1);
    for id in bought_together {
        if let Some(product) = index.get_product(id) {
            println!("  → {}", product.name);
        }
    }

    println!("\n📌 Recomendações de 2º grau (amigos de amigos):");
    graph.connect_bought_together(2, 3, 0.6);
    let depth2_recommendations = graph.get_recommendations_depth_2(1, 5);
    println!("  Conexão adicionada: Mouse → Camiseta (para testar 2º grau)");
    for (product_id, score) in depth2_recommendations {
        if let Some(product) = index.get_product(product_id) {
            println!("  → {} | Score combinado: {:.3}", product.name, score);
        }
    }

    println!("\n✅ Algoritmo de recomendação baseado em grafo implementado!");
    println!("\n📊 Complexidade:");
    println!("  • Recomendações diretas: O(E) onde E = arestas do nó");
    println!("  • Recomendações 2º grau: O(E²) no pior caso");
    println!("  • Busca por tipo: O(E) com filtragem");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 Motor de Busca Integrado");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut search_engine = SearchEngine::new();

    let mut se_product1 = Product::new(
        1,
        "Notebook Dell Inspiron".to_string(),
        "Notebook para trabalho e estudos".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        3500.0,
    );
    se_product1.add_tag("laptop".to_string());
    se_product1.rating = 4.5;

    let mut se_product2 = Product::new(
        2,
        "Mouse Gamer Logitech".to_string(),
        "Mouse com RGB e alta precisão".to_string(),
        "Logitech".to_string(),
        Category::Electronics,
        250.0,
    );
    se_product2.add_tag("gaming".to_string());
    se_product2.rating = 4.8;

    let mut se_product3 = Product::new(
        3,
        "Camiseta Nike Dri-Fit".to_string(),
        "Camiseta esportiva com tecnologia Dri-Fit".to_string(),
        "Nike".to_string(),
        Category::Clothing,
        120.0,
    );
    se_product3.add_tag("esporte".to_string());
    se_product3.rating = 4.3;

    let mut se_product4 = Product::new(
        4,
        "Notebook Asus Vivobook".to_string(),
        "Notebook fino e leve para o dia a dia".to_string(),
        "Asus".to_string(),
        Category::Electronics,
        2800.0,
    );
    se_product4.add_tag("laptop".to_string());
    se_product4.rating = 4.2;

    search_engine.add_product(se_product1);
    search_engine.add_product(se_product2);
    search_engine.add_product(se_product3);
    search_engine.add_product(se_product4);

    println!("\n📊 Status do motor de busca:");
    println!("  • Produtos indexados: {}", search_engine.get_product_count());
    let (nodes, edges) = search_engine.get_graph_stats();
    println!("  • Grafo: {} nós, {} arestas", nodes, edges);

    println!("\n🔍 Busca básica por 'notebook':");
    let results = search_engine.basic_search("notebook");
    for (i, result) in results.iter().enumerate().take(3) {
        println!("  {}. {} | Score: {:.2} | Tipo: {:?}",
                 i+1, result.product.name, result.score, result.match_type);
    }

    println!("\n🔍 Busca por categoria Electronics:");
    let category_results = search_engine.search_by_category(&Category::Electronics);
    for (i, result) in category_results.iter().enumerate().take(3) {
        println!("  {}. {} | Score: {:.2} | Rating: ⭐{:.1}",
                 i+1, result.product.name, result.score, result.product.rating);
    }

    println!("\n🔍 Busca por marca 'Dell':");
    let brand_results = search_engine.search_by_brand("Dell");
    for (i, result) in brand_results.iter().enumerate() {
        println!("  {}. {} | Preço: R$ {:.2}",
                 i+1, result.product.name, result.product.price);
    }

    println!("\n✅ Motor de busca básico integrado funcionando!");
    println!("Próximos passos: filtros avançados e recomendações integradas");
}
