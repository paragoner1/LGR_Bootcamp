struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product { // implementation block
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }
    // associated function (does not use self to refer to the struct) call with ::get
    fn get_default_sales_tax() -> f32 { 
        0.1
    }
    // methods (use self to refer to the struct) call with . (dot) notation
    fn calculate_sales_tax(&self) -> f32 {// reference the instance (Product) without taking ownership
        self.price * Product::get_default_sales_tax()
    }
    fn set_price(&mut self, price: f32) { // take a mutable reference to the instance (Product)
        self.price = price;
    }
    fn buy(self) -> i32 { // take ownership of the instance (Product)
        let name = self.name;
        println!("{name} has been bought");
        123
    }
}
fn main() {
    let mut book = Product::new(
        String::from("Book"),
        30.0);
    
      // create an instance of the struct Product using the associated function `new`
    let sales_tax = book.calculate_sales_tax();// call the method on the instance of the struct 
    println!("sales tax: {}", sales_tax);
    book.set_price(1.0);
    book.buy();
}

  
    
    

