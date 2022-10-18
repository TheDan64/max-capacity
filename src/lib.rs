use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;
use once_cell::sync::OnceCell;

pub mod collections;
pub(crate) mod metadata;
pub mod vec;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Uid(u64);

impl Uid {
    pub fn new() -> Self {
        static NEXT_UID: AtomicU64 = AtomicU64::new(0);

        Uid(NEXT_UID.fetch_add(1, Ordering::Relaxed))
    }
}

pub(crate) enum ReportEvent {
    CapacityIncrease(usize, usize),
    CapacityDecrease(usize, usize),
}

pub(crate) static REPORT_DATA: OnceCell<DashMap<Uid, ReportEvent>> = OnceCell::new();

pub struct Report;

impl Report {
    pub fn print() {
        for ref_multi in REPORT_DATA.get_or_init(DashMap::new).iter() {
            let (key, value) = ref_multi.pair();
        }
    }
}
