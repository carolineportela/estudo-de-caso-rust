use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use crate::model::ProductId;

pub struct Recommender {
    graph: Graph<ProductId, f32>,
    node_map: HashMap<ProductId, NodeIndex>,
}

impl Recommender {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ProductId) {
        let idx = self.graph.add_node(id);
        self.node_map.insert(id, idx);
    }

    pub fn add_edge(&mut self, a: ProductId, b: ProductId, weight: f32) {
        let ia = self.node_map[&a];
        let ib = self.node_map[&b];
        self.graph.update_edge(ia, ib, weight);
    }

    pub fn recommend(&self, id: ProductId, top_n: usize) -> Vec<ProductId> {
        let idx = match self.node_map.get(&id) {
            Some(&i) => i,
            None => return vec![],
        };
        let mut neigh: Vec<_> = self.graph
            .neighbors(idx)
            .filter_map(|n| {
                self.graph.find_edge(idx, n).map(|e| {
                    (self.graph[n], *self.graph.edge_weight(e).unwrap())
                })
            })
            .collect();
        neigh.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        neigh.into_iter().take(top_n).map(|(id, _)| id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_recommender() {
        let rec = Recommender::new();
        assert!(rec.recommend(42, 3).is_empty());
    }

    #[test]
    fn test_simple_edge_recommendation() {
        let mut rec = Recommender::new();
        rec.add_node(1);
        rec.add_node(2);
        rec.add_edge(1, 2, 0.8);

        let out = rec.recommend(1, 1);
        assert_eq!(out, vec![2]);
    }

    #[test]
    fn test_weight_ordering() {
        let mut rec = Recommender::new();
        rec.add_node(1);
        rec.add_node(2);
        rec.add_node(3);
        rec.add_edge(1, 2, 0.5);
        rec.add_edge(1, 3, 0.9);
        let out = rec.recommend(1, 2);
        assert_eq!(out, vec![3, 2]);
    }
}

