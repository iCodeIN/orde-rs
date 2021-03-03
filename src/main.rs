mod datastore;

fn main() {
    match datastore::create_order(String::from("Apple"), 3) {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };

    match datastore::show_table() {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };

    match datastore::update_product_amount(String::from("Apple"), 6) {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };

    match datastore::show_table() {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };

    match datastore::create_order(String::from("Cola"), 6) {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };

    match datastore::create_order(String::from("Casio Watch"), 17) {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };

    match datastore::show_table() {
        Err(e) => println!("Error: {}",e),
        Ok(_) => println!("OK")
    };
}
