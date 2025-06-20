use crate::model::Product;
use csv::Reader;
use std::error::Error;
use std::path::Path;

pub fn load_products<P: AsRef<Path>>(path: P) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut products = Vec::new();
    for result in rdr.deserialize() {
        let product: Product = result?;
        products.push(product);
    }
    Ok(products)
}
