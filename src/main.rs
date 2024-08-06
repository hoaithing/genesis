use diesel::prelude::*;
use genesis::*;
use genesis::models::Order;
use genesis::schema::shop_module_order::dsl::*;


fn main() {
    let connection = &mut establish_connection();
    let results = shop_module_order.filter(esim.eq(true))
        .limit(10)
        .select(Order::as_select() )
        .order_by(created.desc())
        .load(connection)
        .expect("Error loading orders");

    println!("Displaying {} orders", results.len());
    for order in results {
        println!("{}", order.receiver_name);
        println!("-----------\n");
        println!("{}", order.created);
    }
}
