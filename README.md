## MegaStore Search

Sistema de Busca Otimizado para Catálogo de Produtos – CLI em Rust

---

## Descrição do projeto
O **MegaStore Search** é um CLI em Rust que:

- Indexa um catálogo de produtos a partir de CSV.
- Realiza busca full-text com normalização (case- e accent-insensitive) e remoção de stop-words.
- Oferece filtros por categoria, marca, tag e faixa de preço.
- Utiliza cache LRU para acelerar buscas repetidas.
- Gera recomendações via grafo de similaridade.
- Possui testes unitários e de integração e benchmarks de performance.

## Tecnologias
- **Rust (2024 edition)**
- Crates:
  - `csv`, `serde`/`serde_json`
  - `clap`
  - `unicode-normalization`
  - `lazy_static`, `regex`
  - `lru`
  - `petgraph`
  - `criterion`

## Como clonar e compilar
```bash
# Clone o repositório e entre na pasta do projeto
git clone https://github.com/carolineportela/mega_store_search.git
cd mega_store_search
code .

# Compile 
cargo build
cargo test
