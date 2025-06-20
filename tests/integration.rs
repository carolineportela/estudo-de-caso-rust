use std::env;
use mega_store_search::loader::load_products;
use mega_store_search::index::InvertedIndex;
use mega_store_search::graph::Recommender;
use mega_store_search::tokenizer::tokenize;

#[test]
fn test_full_flow() {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let csv_path = format!("{}/tests/data/sample.csv", project_dir);
    let products = load_products(&csv_path).expect("Falha ao carregar produtos do CSV");

    let mut inverted_index = InvertedIndex::new();
    for product in &products {
        inverted_index.insert_text(&product.name, product.id);
        inverted_index.insert_text(&product.category, product.id);
        inverted_index.insert_text(&product.brand, product.id);
        for tag in &product.tags {
            inverted_index.insert_text(tag, product.id);
        }
    }

    let name_query = tokenize("mouse");
    let ids_by_name = inverted_index.query(&name_query);
    assert_eq!(ids_by_name, vec![products[0].id]);

    let category_query = tokenize("eletronicos");
    let ids_by_category = inverted_index.query(&category_query);
    let expected_category_ids: Vec<u64> = products
        .iter()
        .filter(|p| tokenize(&p.category).contains(&"eletronicos".to_string()))
        .map(|p| p.id)
        .collect();
    assert_eq!(ids_by_category, expected_category_ids);

    let tag_query = tokenize("gamer");
    let ids_by_tag = inverted_index.query(&tag_query);
    let expected_tag_ids: Vec<u64> = products
        .iter()
        .filter(|p| {
            p.tags
             .iter()
             .any(|t| tokenize(t).contains(&"gamer".to_string()))
        })
        .map(|p| p.id)
        .collect();
    assert_eq!(ids_by_tag, expected_tag_ids);

    let mut recommender = Recommender::new();
    for product in &products {
        recommender.add_node(product.id);
    }
    recommender.add_edge(products[0].id, products[1].id, 1.0);
    let recommended_ids = recommender.recommend(products[0].id, 1);
    assert_eq!(recommended_ids, vec![products[1].id]);
}

