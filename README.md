# 🛍️ Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

<div align="center">

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Performance](https://img.shields.io/badge/Performance-1.1M%20products%2Fsec-green)](./PERFORMANCE_RESULTS.md)

Sistema de busca e recomendação de alta performance para e-commerce, implementado em Rust com indexação HashMap O(1) e recomendações baseadas em grafos.

</div>

## 📋 Índice

- [Visão Geral](#-visão-geral)
- [Características](#-características)
- [Tecnologias Utilizadas](#-tecnologias-utilizadas)
- [Instalação](#-instalação)
- [Como Executar](#-como-executar)
- [Executar Testes](#-executar-testes)
- [Exemplos de Uso](#-exemplos-de-uso)
- [Arquitetura do Sistema](#-arquitetura-do-sistema)
- [Algoritmos e Estruturas de Dados](#-algoritmos-e-estruturas-de-dados)
- [Performance e Escalabilidade](#-performance-e-escalabilidade)
- [API Documentation](#-api-documentation)
- [Contribuindo](#-contribuindo)
- [Licença](#-licença)

## 🎯 Visão Geral

O **MegaStore Search System** é uma solução de busca e recomendação otimizada para catálogos de produtos em larga escala. Desenvolvido para resolver os desafios de performance e precisão em e-commerce com milhões de produtos, o sistema oferece:

- **Busca ultrarrápida**: Respostas em microssegundos
- **Recomendações inteligentes**: Baseadas em grafos de relacionamento
- **Escalabilidade linear**: Performance consistente com crescimento do catálogo
- **Filtros avançados**: Múltiplos critérios simultâneos

### 🎯 Problema Resolvido

A MegaStore enfrentava:
- ❌ Buscas lentas e imprecisas
- ❌ Resultados irrelevantes
- ❌ Perda de vendas por frustração dos clientes

Nossa solução oferece:
- ✅ Busca O(1) com HashMap indexing
- ✅ Recomendações baseadas em comportamento
- ✅ Performance de 1.1M produtos/segundo

## ✨ Características

### 🔍 Sistema de Busca
- **Busca por nome** com scoring de relevância
- **Busca por marca, categoria e tags**
- **Filtros avançados** (preço, rating, estoque)
- **Busca híbrida** combinando múltiplos critérios

### 🤖 Sistema de Recomendação
- **Grafo de relacionamentos** entre produtos
- **4 tipos de relações**: Similar, BoughtTogether, SameCategory, SameBrand
- **Recomendações de 2º grau** (amigos de amigos)
- **Scoring ponderado** por tipo de relação

### ⚡ Performance
- **Indexação**: 1.1M produtos/segundo
- **Busca por ID**: 5.6 nanosegundos
- **Busca geral**: < 20 microssegundos
- **Escalabilidade linear** comprovada

## 🛠 Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** - Linguagem de programação de sistemas
- **[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)** - Indexação O(1)
- **[petgraph](https://github.com/petgraph/petgraph)** - Estruturas de grafo
- **[serde](https://serde.rs/)** - Serialização/Deserialização
- **[rayon](https://github.com/rayon-rs/rayon)** - Paralelização
- **[criterion](https://github.com/bheisler/criterion.rs)** - Benchmarking

### Dependências Principais

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
indexmap = "2.0"
petgraph = "0.6"
rayon = "1.7"
criterion = "0.5"
```

## 📦 Instalação

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) 1.70 ou superior
- Cargo (incluído com Rust)

### Clonar o Repositório

```bash
git clone https://github.com/barbmarcio/megastore-search.git
cd megastore-search
```

### Compilar o Projeto

```bash
cargo build --release
```

## 🚀 Como Executar

### Executar o Sistema de Demonstração

```bash
cargo run --release
```

### Executar com Dados Customizados

```rust
use megastore_search::{SearchEngine, Product, Category};

fn main() {
    let mut engine = SearchEngine::new();

    // Adicionar produtos
    let product = Product::new(
        1,
        "Notebook Dell XPS".to_string(),
        "High-performance laptop".to_string(),
        "Dell".to_string(),
        Category::Electronics,
        1500.0,
    );
    engine.add_product(product);

    // Buscar produtos
    let results = engine.basic_search("notebook");
    println!("Found {} products", results.len());
}
```

## 🧪 Executar Testes

### Testes Unitários

```bash
# Executar todos os testes
cargo test

# Executar testes específicos
cargo test test_product_creation
cargo test index_tests
cargo test graph_tests

# Com output detalhado
cargo test -- --nocapture
```

### Testes de Performance

```bash
# Executar todos os benchmarks
cargo bench

# Executar benchmark específico
cargo bench indexing_benchmark

# Executar script de benchmarks
./run_benchmarks.sh
```

## 📖 Exemplos de Uso

### Busca Básica

```rust
let results = engine.basic_search("gaming laptop");
for result in results {
    println!("{} - Score: {}", result.product.name, result.score);
}
```

### Busca com Filtros

```rust
use megastore_search::SearchFilters;

let filters = SearchFilters::new()
    .category(Category::Electronics)
    .price_range(500.0, 2000.0)
    .min_rating(4.0)
    .in_stock_only();

let results = engine.search_with_filters(Some("laptop"), &filters);
```

### Recomendações

```rust
// Produtos similares
let similar = engine.search_similar_products(product_id);

// Frequentemente comprados juntos
let bought_together = engine.get_frequently_bought_together(product_id);

// Recomendações com score
let recommendations = engine.get_recommendations_for_product(product_id, 10);
```

### Busca Híbrida

```rust
// Combina busca textual com recomendações
let results = engine.search_with_recommendations("gaming", true, 20);

// Busca avançada com filtros e recomendações
let hybrid_results = engine.hybrid_search(
    Some("laptop"),
    &filters,
    true  // incluir recomendações
);
```

## 🏗 Arquitetura do Sistema

```
megastore-search/
├── src/
│   ├── main.rs              # Aplicação de demonstração
│   ├── lib.rs               # Biblioteca principal
│   ├── models/              # Estruturas de dados
│   │   ├── mod.rs
│   │   └── product.rs       # Modelo Product e Category
│   ├── indexing/            # Sistema de indexação
│   │   └── mod.rs           # ProductIndex com HashMap
│   ├── graph/               # Grafo de recomendações
│   │   └── mod.rs           # RecommendationGraph
│   └── search/              # Motor de busca
│       └── mod.rs           # SearchEngine integrado
├── tests/                   # Testes unitários
│   ├── product_tests.rs
│   ├── index_tests.rs
│   ├── graph_tests.rs
│   ├── search_tests.rs
│   └── integration_tests.rs
├── benches/                 # Benchmarks de performance
│   ├── indexing_benchmark.rs
│   ├── graph_benchmark.rs
│   └── search_benchmark.rs
└── Cargo.toml              # Configuração do projeto
```

### Componentes Principais

1. **Product Model**: Estrutura de dados para produtos com categorias, tags e métodos de scoring
2. **ProductIndex**: Sistema de indexação com múltiplos HashMap para busca O(1)
3. **RecommendationGraph**: Grafo não-direcionado para relações entre produtos
4. **SearchEngine**: Motor integrado que combina indexação e recomendações

## 🔧 Algoritmos e Estruturas de Dados

### HashMap Indexing

```rust
pub struct ProductIndex {
    products: IndexMap<u64, Product>,       // O(1) lookup
    name_index: HashMap<String, HashSet<u64>>,    // Busca por nome
    brand_index: HashMap<String, HashSet<u64>>,   // Busca por marca
    category_index: HashMap<Category, HashSet<u64>>, // Busca por categoria
    tag_index: HashMap<String, HashSet<u64>>,     // Busca por tag
}
```

**Complexidade:**
- Inserção: O(k) onde k = número de palavras/tags
- Busca: O(1) para acesso direto
- Remoção: O(k) para limpar índices

### Grafo de Recomendações

```rust
pub struct RecommendationGraph {
    graph: UnGraph<ProductNode, EdgeWeight>,
    product_to_node: HashMap<u64, NodeIndex>,
}
```

**Algoritmos:**
- **BFS** para recomendações diretas: O(V + E)
- **2-hop traversal** para recomendações indiretas: O(V²) no pior caso
- **Scoring ponderado** por tipo de relação

### Score de Relevância

```rust
score = base_score * (1.0 + rating / 10.0)

// Pesos por tipo de match:
// - Nome exato: 10 pontos
// - Marca: 5 pontos
// - Descrição: 2 pontos
// - Tag: 3 pontos
```

## 📊 Performance e Escalabilidade

### Resultados dos Benchmarks

| Métrica | Performance | Comparação com Meta |
|---------|-------------|-------------------|
| **Indexação** | 1.1M produtos/seg | 110x melhor |
| **Busca por ID** | 5.6 ns | 178,000x melhor |
| **Busca geral** | < 20 µs | 5,000x melhor |
| **Recomendações** | < 50 ms | ✅ Dentro da meta |

### Análise de Escalabilidade

```
Dataset     Indexação    Busca      Memória
100         77 µs        5 ns       ~1 KB
1,000       766 µs       5 ns       ~10 KB
10,000      8.8 ms       5 ns       ~100 KB
100,000     ~88 ms       5 ns       ~1 MB
1,000,000   ~880 ms      5 ns       ~10 MB
```

**Características:**
- ✅ Escalabilidade linear para indexação
- ✅ O(1) mantido independente do tamanho
- ✅ Uso eficiente de memória

### Projeções para Produção

Para **1 milhão de produtos**:
- Tempo de indexação: ~15 minutos (setup inicial)
- Busca: ainda < 20 µs
- Memória: ~500MB-1GB
- Throughput: 50,000+ buscas/segundo

## 📚 API Documentation

### SearchEngine

```rust
// Criar engine
let mut engine = SearchEngine::new();

// Adicionar produto
engine.add_product(product);

// Busca básica
engine.basic_search(query: &str) -> Vec<SearchResult>

// Busca com filtros
engine.search_with_filters(
    query: Option<&str>,
    filters: &SearchFilters
) -> Vec<SearchResult>

// Recomendações
engine.get_recommendations_for_product(
    product_id: u64,
    limit: usize
) -> Vec<SearchResult>

// Busca híbrida
engine.hybrid_search(
    query: Option<&str>,
    filters: &SearchFilters,
    use_recommendations: bool
) -> Vec<SearchResult>
```

### SearchFilters

```rust
let filters = SearchFilters::new()
    .price_range(min, max)
    .min_rating(rating)
    .category(category)
    .brand(brand)
    .add_tag(tag)
    .in_stock_only();
```

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

### Guidelines

- Mantenha a cobertura de testes acima de 80%
- Execute `cargo fmt` antes do commit
- Execute `cargo clippy` e resolva warnings
- Adicione benchmarks para novas features
- Atualize a documentação conforme necessário

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 👥 Autores

- **Marcio Barbosa** - *Desenvolvimento inicial* - [GitHub](https://github.com/barbmarcio)
