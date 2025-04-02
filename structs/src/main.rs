struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

fn main() {
    let mut book = Product {
        name: String::from("Book"),
        price: 288888.85,
        in_stock: true,
    };
    let sales_tax = calculate_sales_tax(&book); 
    println!("sales tax: {}", sales_tax);

    fn calculate_sales_tax(product: &Product) -> f32 {
        // Calculate sales tax based on the product price
        let tax_rate = 0.10; 
        product.price * tax_rate
    }
    
}
 