struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {
    Books,
    Clothing,
    Electronics,
}
fn main() {
    let category = ProductCategory::Electronics;
    let product = Product {
        name: String::from("TV"),
        category,
        price: 200.98,
        in_stock: true,
    };

}