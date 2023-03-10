use shopify_api_client_cli::models::product_list::ProductList;

pub fn render_products_info(product_list: &ProductList) {
    println!();
    println!("{}", format!("{:-<45}", ""));
    println!("Unifi 團購清單");
    product_list.items().iter().for_each(|product| {
        println!(
            "{}). Id: {}, name: {}, price: {}, description: {}",
            product.serial_number(),
            product.id(),
            product.name(),
            product.price(),
            product.description()
        );
    });
    println!("{}", format!("{:-<45}", ""));
}
