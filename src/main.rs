use megastore_search::{Product, Category, ProductIndex, RecommendationGraph};

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

    println!("Nós no grafo: {}", graph.product_count());
    println!("Arestas no grafo: {}", graph.edge_count());

    println!("\n✅ Estrutura básica do grafo criada com sucesso!");
}
