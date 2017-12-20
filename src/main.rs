extern crate trayder;

use trayder::orderbook;

pub fn main() {
    let mut ob = orderbook::Orderbook::new();
    ob.limit(5_f64, 1_f64);
    ob.limit(5_f64, 1_f64);
    ob.limit(5_f64, 1_f64);
    ob.limit(4_f64, 1_f64);
    ob.limit(3_f64, 1_f64);
    ob.limit(6_f64, -1_f64);
    ob.limit(6_f64, -1_f64);
    ob.limit(7_f64, -1_f64);
    ob.limit(8_f64, -1_f64);

    // println!("{:?}", ob.best_bid());
    // println!("{:?}", ob.best_offer());
    // println!("{:#?}", ob);

    ob.limit(6_f64, 1_f64);
    // println!("{:#?}", ob);
}
