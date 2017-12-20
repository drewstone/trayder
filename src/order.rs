use std::cmp::Ordering;
use std::ops::{Add,Sub,Mul,Div};
use chrono::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct F64(pub f64);

impl Sub for F64 {
    type Output = F64;
    fn sub(self, other: F64) -> F64 {
        F64(self.0 - other.0)
    }
}

impl Add for F64 {
    type Output = F64;
    fn add(self, other: F64) -> F64 {
        F64(self.0 + other.0)
    }
}

impl Mul for F64 {
    type Output = F64;
    fn mul(self, other: F64) -> F64 {
        F64(self.0 * other.0)
    }
}

impl Div for F64 {
    type Output = F64;
    fn div(self, other: F64) -> F64 {
        F64(self.0 / other.0)
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &F64) -> bool {
        if self.0 == other.0 {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &F64) -> Option<Ordering> {
        // Kinda hacky, but I think this should work...
        if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &F64) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub struct Order {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub volume: F64
}

impl Ord for Order {
    fn cmp(&self, other: &Order) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Order) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Order {
    pub fn new(id: u64, ts: DateTime<Utc>, vol: f64) -> Order {
        Order {
            id: id,
            timestamp: ts,
            volume: F64(vol)
        }
    }
}
