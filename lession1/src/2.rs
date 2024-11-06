use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

struct Store {
    products: HashMap<String, Product>,
}

impl Store {
    fn new() -> Store {
        Store {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, price: f64, quantity: u32) -> Result<(), &'static str> {
        if price <= 0.0 || quantity == 0 {
            return Err("Invalid price or quantity");
        }
        if self.products.contains_key(&name) {
            return Err("Product already exists");
        }
        self.products.insert(name.clone(), Product { name, price, quantity });
        Ok(())
    }

    fn update_product(&mut self, name: &str, new_price: Option<f64>, new_quantity: Option<u32>) -> Result<(), &'static str> {
        if let Some(product) = self.products.get_mut(name) {
            if let Some(price) = new_price {
                if price <= 0.0 {
                    return Err("Price must be greater than zero");
                }
                product.price = price;
            }

            if let Some(quantity) = new_quantity {
                if quantity == 0 {
                    return Err("Quantity must be greater than zero");
                }
                product.quantity = quantity;
            }

            Ok(())
        } else {
            Err("Product not found.")
        }
    }

    fn find_product(&self, name: &str) -> Option<&Product> {
        return self.products.get(name)
    }

    fn remove_product(&mut self, name: &str) -> Result<(), &'static str> {
        if self.products.remove(name).is_none() {
            Err("Product not found.")
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut store = Store::new();

    store.add_product("Макбук".to_string(), 1999.0, 10).unwrap();
    store.add_product("Айфон".to_string(), 999.0, 100).unwrap();

    store.update_product("Макбук", Some(2000.1), Some(8)).unwrap();

    if let Some(product) = store.find_product("Айфон") {
        println!("{:?}", product);
    }

    store.remove_product("Айфон").unwrap();

    if let Some(product) = store.find_product("Айфон") {
        println!("{:?}", product);
    } else {
        println!("Айфон not found");
    }
}