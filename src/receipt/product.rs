pub fn create_products() -> Vec<StoreProduct> {
    vec![
        StoreProduct {
            name: String::from("Zbox 720"),
            price: 600,
        },
        StoreProduct {
            name: String::from("GPU - AND Random RT6600"),
            price: 200,
        },
        StoreProduct {
            name: String::from("Potato"),
            price: 1,
        },
    ]
}
#[derive(Eq, Hash, PartialEq)]
pub struct StoreProduct {
    pub name: String,
    pub price: i32,
}