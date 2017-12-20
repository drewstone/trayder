use std::f64::{MIN_POSITIVE,MAX};
use std::collections::{BTreeMap,BinaryHeap};
use std::result::Result;
use chrono::prelude::*;
use order::{Order,F64};

type Pricepoint = F64;
type Orderlist = BinaryHeap<Order>;

#[derive(Debug, Clone)]
pub struct Orderbook {
    pub bids: BTreeMap<Pricepoint, Orderlist>,
    pub asks: BTreeMap<Pricepoint, Orderlist>,
    pub count: u64,
    pub best_bid: F64,
    pub best_offer: F64
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            count: 0,
            best_bid: F64(MIN_POSITIVE),
            best_offer: F64(MAX)
        }
    }

    pub fn market(&mut self, volume: f64) -> Result<DateTime<Utc>, &'static str> {
        if volume == 0_f64 {
            return Err("Invalid market order volume");
        }

        let id = self.count;
        let order_time = Utc::now();
        let (search, side) = if volume.is_sign_positive() {(&mut self.asks, &mut self.bids)} else {(&mut self.bids, &mut self.asks)};
        let mut order = Order::new(id, order_time, volume.abs());
        let bbool = if volume.is_sign_positive() {F64(1_f64)} else {F64(-1_f64)};
        let mut vol = F64(volume);
        let mut iter = search.iter_mut();

        loop {
            let mapping = if volume.is_sign_positive() {iter.next_back()} else {iter.next()};
            match mapping {
                Some((p,list)) => {
                    while list.len() > 0 && vol != F64(0_f64) {
                        if vol >= list.peek().unwrap().volume {
                            let order = list.pop().unwrap();
                            vol = vol - order.volume * bbool;
                        } else {
                            let mut val = list.peek_mut().unwrap();
                            (*val).volume = val.volume - vol * bbool;
                            vol = F64(0_f64);
                        }
                    }

                    if vol == F64(0_f64) {
                        break;
                    }
                },
                None => {
                    order = Order::new(id, order_time, vol.0.abs());
                    break;
                }
            }
        }

        Ok(order_time)
    }

    pub fn limit(&mut self, price: f64, volume: f64) -> Result<DateTime<Utc>, &'static str> {
        if volume == 0_f64 {
            return Err("Invalid limit order volume");
        }

        let id = self.count;
        let ppoint = F64(price);
        let order_time = Utc::now();
        let (search, side) = if volume.is_sign_positive() {(&mut self.asks, &mut self.bids)} else {(&mut self.bids, &mut self.asks)};
        let mut order = Order::new(id, order_time, volume.abs());
        let bbool = if volume.is_sign_positive() {F64(1_f64)} else {F64(-1_f64)};
        let marketable = if volume.is_sign_positive() {F64(price) >= self.best_offer} else {F64(price) <= self.best_bid};

        if marketable {
            let mut vol = F64(volume);
            let mut iter = search.iter_mut();
            loop {
                let mapping = if volume.is_sign_positive() {iter.next_back()} else {iter.next()};
                match mapping {
                    Some((p,list)) => {

                        loop {
                            let inrange = if volume.is_sign_positive() && list.len() > 0  {&ppoint >= p} else {&ppoint <= p};
                            if inrange {
                                if vol >= list.peek().unwrap().volume {
                                    let order = list.pop().unwrap();
                                    vol = vol - order.volume * bbool;
                                } else {
                                    let mut val = list.peek_mut().unwrap();
                                    (*val).volume = val.volume - vol * bbool;
                                    vol = F64(0_f64);
                                    break;
                                }
                            } else {
                                break;
                            }

                        }

                        if vol == F64(0_f64) {
                            break;
                        }
                    },
                    None => {
                        order = Order::new(id, order_time, vol.0.abs());
                    }
                }
            }
        }

        if order.volume > F64(0_f64) {
            if side.contains_key(&ppoint) {
                if let Some(list) = side.get_mut(&ppoint) {
                    (*list).push(order);
                }
            } else {
                let mut list = Orderlist::new();
                list.push(order);
                side.insert(ppoint, list);
            }
        }

        if volume.is_sign_positive() {
            self.best_bid = match side.iter().next_back() {
                Some((price,_)) => *price,
                None => F64(MIN_POSITIVE)
            };
        } else {
            self.best_offer = match side.iter().next() {
                Some((price,_)) => *price,
                None => F64(MAX)
            };
        }


        self.count += 1;
        Ok(order_time)
    }

    pub fn best_bid(&mut self) -> F64 {
        match self.bids.iter().next_back() {
            Some((price,_)) => *price,
            None => F64(MIN_POSITIVE)
        }
    }

    pub fn best_offer(&mut self) -> F64 {
        match self.asks.iter().next() {
            Some((price,_)) => *price,
            None => F64(MAX)
        }
    }

    pub fn get_bid_volume(&mut self) -> F64 {
        match self.bids.get(&self.best_bid) {
            Some(list) => {
                println!("{:#?}, {:#?}", list, self.best_bid);
                list.iter()
                    .map(|o| o.volume)
                    .fold(F64(0_f64), |a,b| a+b)
            },
            None => F64(0_f64)
        }
    }
}
