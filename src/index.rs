use std::collections::HashMap;
use crate::model::ProductId;
use crate::tokenizer::tokenize;

pub struct InvertedIndex {
    map: HashMap<String, Vec<ProductId>>,
}

impl InvertedIndex {

    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, token: String, id: ProductId) {
        self.map.entry(token).or_default().push(id);
    }

    pub fn insert_text(&mut self, text: &str, id: ProductId) {
        for token in tokenize(text) {
            self.insert(token, id);
        }
    }
    
        pub fn query(&self, tokens: &[String]) -> Vec<ProductId> {
            let mut result = tokens.iter()
                .filter_map(|t| self.map.get(t))
                .fold(None, |acc: Option<Vec<ProductId>>, vec| {
                    Some(match acc {
                        None    => vec.clone(),
                        Some(prev) => prev.into_iter()
                                        .filter(|x| vec.contains(x))
                                        .collect(),
                    })
                })
                .unwrap_or_default();
            result.sort_unstable();
            result.dedup();
            result
        }

} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_token() {
        let mut idx = InvertedIndex::new();
        idx.insert("foo".into(), 1);
        idx.insert("bar".into(), 2);
        let res = idx.query(&vec!["foo".to_string()]);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn test_intersection_of_two_tokens() {
        let mut idx = InvertedIndex::new();
        idx.insert("alpha".into(), 1);
        idx.insert("beta".into(), 1);
        idx.insert("beta".into(), 2);

        let res = idx.query(&vec!["alpha".to_string(), "beta".to_string()]);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn test_no_match_returns_empty() {
        let mut idx = InvertedIndex::new();
        idx.insert("x".into(), 1);
        let res = idx.query(&vec!["y".to_string()]);
        assert!(res.is_empty());
    }

    #[test]
    fn test_insert_text() {
        let mut idx = InvertedIndex::new();
        idx.insert_text("Foo-Bar baz", 99);
        assert_eq!(idx.query(&vec!["foo".into()]), vec![99]);
        assert_eq!(idx.query(&vec!["bar".into(), "baz".into()]), vec![99]);
    }

}


