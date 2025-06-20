use clap::Parser;
use std::env;
use unicode_normalization::UnicodeNormalization;
use mega_store_search::{index, graph};
use mega_store_search::loader::load_products;
use mega_store_search::tokenizer::tokenize;
use lru::LruCache;


fn normalize(s: &str) -> String {
    s.nfkd()
     .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
     .collect::<String>()
     .to_lowercase()
}

#[derive(Parser)]
#[command(name = "mega_store_search")]
enum Command {
    Search {

        query: String,
        #[arg(long)]
        category_filter: Option<String>,
        #[arg(long)]
        brand_filter: Option<String>,
        #[arg(long)]
        tag_filter: Option<String>,
        #[arg(long)]
        price_min: Option<f32>,
        #[arg(long)]
        price_max: Option<f32>,
    },

    Recommend {
        product_id: u64,
        #[arg(long)]
        top_n: Option<usize>,
    },
}

fn main() {
    let mut search_cache: LruCache<String, Vec<u64>> = LruCache::new(128);
    let cmd = Command::parse();

    match cmd {
        Command::Search {
            query,
            category_filter,
            brand_filter,
            tag_filter,
            price_min,
            price_max,
        } => {
            println!("Buscando por “{}”…", query);

            let root = env!("CARGO_MANIFEST_DIR");
            let path = format!("{}/tests/data/sample.csv", root);
            let products = match load_products(&path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(" Erro ao carregar CSV ({}): {}", path, e);
                    std::process::exit(1);
                }
            };

            let mut idx = index::InvertedIndex::new();
            for p in &products {
                idx.insert_text(&p.name, p.id);
                idx.insert_text(&p.category, p.id);
                idx.insert_text(&p.brand, p.id);
                for tg in &p.tags {
                    idx.insert_text(tg, p.id);
                }
            }

            let tokens = tokenize(&query);
            let key = tokens.join(" ");

            let raw_ids = if let Some(cached) = search_cache.get(&key) {
                println!("(via cache)");
                cached.clone()
            } else {
                let res = idx.query(&tokens);
                search_cache.put(key.clone(), res.clone());
                res
            };

            let final_ids: Vec<u64> = raw_ids
                .into_iter()
                .filter(|&id| {
                    let p = products.iter().find(|p| p.id == id).unwrap();

                    if let Some(ref cf) = category_filter {
                        if normalize(&p.category) != normalize(cf) {
                            return false;
                        }
                    }
                    if let Some(ref bf) = brand_filter {
                        if normalize(&p.brand) != normalize(bf) {
                            return false;
                        }
                    }
      
                    if let Some(ref tf) = tag_filter {
                        let ok = p.tags.iter().any(|tt| normalize(tt) == normalize(tf));
                        if !ok {
                            return false;
                        }
                    }
                    if let Some(min) = price_min {
                        if p.price < min {
                            return false;
                        }
                    }
                    if let Some(max) = price_max {
                        if p.price > max {
                            return false;
                        }
                    }

                    true
                })
                .collect();

            println!("Resultados: {:?}", final_ids);
        }

        Command::Recommend { product_id, top_n } => {
            let n = top_n.unwrap_or(5);
            println!("Recomendando top {} para produto {}…", n, product_id);

            let root = env!("CARGO_MANIFEST_DIR");
            let path = format!("{}/tests/data/sample.csv", root);
            let products = match load_products(&path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(" Erro ao carregar CSV ({}): {}", path, e);
                    std::process::exit(1);
                }
            };

            let mut rec = graph::Recommender::new();
            for p in &products {
                rec.add_node(p.id);
            }
            for a in &products {
                for b in &products {
                    if a.id != b.id && a.category == b.category {
                        rec.add_edge(a.id, b.id, 1.0);
                    }
                }
            }

            let suggestions = rec.recommend(product_id, n);
            println!("Sugestões: {:?}", suggestions);
        }
    }
}

