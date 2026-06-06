use super::product::StoreProduct;
use super::product::create_products;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;


struct ReceiptContent {
    products: Vec<StoreProduct>,
    store: String,
}

const PRODUCT_1_NAME: &str = "Xbox 720";
const PRODUCT_2_NAME: &str = "GPU - AND Random RT6600";
const PRODUCT_3_NAME: &str = "Potato";



pub fn start_shopping() {
    let products = create_products();

    let mut receipt_vektor = ReceiptContent {
    products: Vec::new(), 
    store: String::from("Best Buy"),
};

    loop {
        println!("| 1) Add to cart | 2) Remove most recent product | 3) Purchase |");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim(); 
        match input{
            "1" => {
                println!("Which product would you like to add?'");
                for i in 0..products.len() {
                    println!("{}) {} | Price - {}", i + 1, products[i].name, products[i].price);
                }

                let mut product_input = String::new();
                std::io::stdin().read_line(&mut product_input).expect("Failed to read line");
                let product_input = product_input.trim();
                match product_input {
                    "1" => println!("Added {} to cart", PRODUCT_1_NAME),
                    "2" => println!("Added {} to cart", PRODUCT_2_NAME),
                    "3" => println!("Added {} to cart", PRODUCT_3_NAME),
                    _ => println!("Invalid product selection"),
                }
            },
            "2" => {println!("Removed most recent product from cart");
                    if !receipt_vektor.products.is_empty() {
                        receipt_vektor.products.pop();
                    } else {
                        println!("Cart is already empty");
                    }
            },
            "3" => {
               complete_purchase(&receipt_vektor);
                break;
            },
            _ => println!("Invalid input, please try again"),
        }
    }
    
}
fn complete_purchase(receipt: &ReceiptContent) -> Result<(), std::io::Error> {
    let mut file = File::create("receipt.txt").expect("Could not create file");
    let mut total_price = 0;
    let mut product_count:HashMap<&StoreProduct, i32> = HashMap::new();
    for product in &receipt.products {
        if product_count.contains_key(product) {
            *product_count.get_mut(product).unwrap() += 1;
        } else {
            product_count.insert(product, 1);
        }
    }
    writeln!(file, "{}", receipt.store).expect("writing to file failed");
    writeln!(file, "------------------------------").expect("writing to file failed");
    for (product, count) in &product_count {
        writeln!(file, "{} - ${} x {}", product.name, product.price, count).expect("writing to file failed");
        total_price += product.price * *count;
    }
    writeln!(file, "------------------------------").expect("writing to file failed");
    writeln!(file, "Final Price: ${}", total_price).expect("writing to file failed");
    writeln!(file, "------------------------------").expect("writing to file failed");
    Ok(())
}


