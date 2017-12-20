extern crate chrono;

pub mod order;
pub mod orderbook;

#[cfg(test)]
mod tests {
    use super::order::{Order,F64};
    use super::orderbook::Orderbook;
    use super::chrono::prelude::*;
    use super::std::{thread,time};

    #[test]
    fn order_creation() {
        let ts: DateTime<Utc> = Utc.ymd(2000,1,10).and_hms(1,2,3);
        let vol: f64 = 10_f64;

        let order = Order::new(0, ts, vol);
        assert_eq!(order.id, 0);
        assert_eq!(order.timestamp, Utc.ymd(2000,1,10).and_hms(1,2,3));
        assert_eq!(order.volume, F64(vol));
    }

    #[test]
    fn order_comparison() {
        let ten_millis = time::Duration::from_millis(10);
        let vol: f64 = 10_f64;

        let mut ob = Orderbook::new();
        let mut curr_time = Utc::now();

        for _ in 0..5 {
            thread::sleep(ten_millis);
            let _ = ob.limit(5.01_f64, vol).unwrap();
        }

        assert_eq!(ob.count, 5);
        assert_eq!(ob.best_bid(), F64(5.01_f64));

        if let Some(list) = ob.bids.get_mut(&F64(5.01)) {
            loop {
                let o = match list.pop() {
                    Some(order) => order,
                    None => break
                };

                assert!(o.timestamp > curr_time);
                curr_time = o.timestamp;
            }
        }

        assert_eq!(ob.get_bid_volume(), F64(50_f64));
    }

    #[test]
    fn order_execution() {
        let ten_millis = time::Duration::from_millis(10);
        let vol: f64 = 10_f64;

        let mut ob = Orderbook::new();

        for _ in 0..5 {
            thread::sleep(ten_millis);
            let _ = ob.limit(5_f64, vol).unwrap();
        }

        let _ = ob.limit(4_f64, -15_f64).unwrap();
    }
}
