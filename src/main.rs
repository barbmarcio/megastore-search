use megastore_search::models::{Product, Category};

fn main() {
    println!("🛍️ MegaStore Search System");
    println!("Sistema de busca otimizado para catálogo de produtos");

    let product = Product::new(
        1,
        "Notebook Dell".to_string(),
        "Notebook para trabalho".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        3500.0,
    );

    println!("\nProduto teste criado: {} - R$ {:.2}", product.name, product.price);
}
