pub type ProductId = u64;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub category: String,
    pub brand: String,
    pub price: f32,
    #[serde(default)] pub tags: Vec<String>,
}
