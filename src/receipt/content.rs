use super::product::StoreProduct;
use super::product::create_products;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;



pub struct ReceiptContent {
    pub products: Vec<StoreProduct>,
    pub store: String,
}

#[allow(dead_code)]
const PRODUCT_1_NAME: &str = "Xbox 720";
#[allow(dead_code)]
const PRODUCT_2_NAME: &str = "GPU - AND Random RT6600";
#[allow(dead_code)]
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
                println!("Which product would you like to add?");
                for i in 0..products.len() {
                    println!("{}) {} | Price - {}", i + 1, products[i].name, products[i].price);
                }

                let mut product_input = String::new();
                std::io::stdin().read_line(&mut product_input).expect("Failed to read line");
                let product_input = product_input.trim();
                match product_input {
                    // This is from chatgpt becouse i couldnt figure out how to do it without hardcoding it.
                    "1" => receipt_vektor.products.push(StoreProduct { name: products[0].name.clone(), price: products[0].price }),
                    "2" => receipt_vektor.products.push(StoreProduct { name: products[1].name.clone(), price: products[1].price }),
                    "3" => receipt_vektor.products.push(StoreProduct { name: products[2].name.clone(), price: products[2].price }),
                    _ => println!("Invalid product selection"),
                }
            },
            "2" => {
                    if !receipt_vektor.products.is_empty() {
                        receipt_vektor.products.pop();
                    } else {
                        println!("Cart is already empty");
                    }
            },
            "3" => {
               let _ = complete_purchase(&receipt_vektor);
               println!("Thank you for your purchase!");
                break;
            },
            _ => println!("Invalid input, please try again"),
        }
    }
    
}
pub fn complete_purchase(receipt: &ReceiptContent) -> Result<(), std::io::Error> {
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
        writeln!(file, "{} ({}) - {}€", product.name, count, product.price).expect("writing to file failed");
        total_price += product.price * *count;
    }
    writeln!(file, "------------------------------").expect("writing to file failed");
    writeln!(file, "Final Price: {}€", total_price).expect("writing to file failed");
    writeln!(file, "------------------------------").expect("writing to file failed");
    Ok(())
}


