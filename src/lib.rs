use std::ops::{Index, IndexMut};
use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;
use once_cell::sync::OnceCell;

pub mod collections;
pub mod vec;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[derive(Default)]
pub(crate) struct SubReport {
    pub ty_name: String,
    pub events: Vec<ReportEvent>,
}

static REPORT_DATA: OnceCell<DashMap<Uid, SubReport>> = OnceCell::new();

pub struct Report;

impl Report {
    pub(crate) fn insert(id: Uid) {
        REPORT_DATA
            .get_or_init(DashMap::new)
            .insert(id, SubReport::default());
    }

    pub fn print() {
        for ref_multi in REPORT_DATA.get_or_init(DashMap::new).iter() {
            let (id, events) = ref_multi.pair();
        }
    }
}

impl Index<Uid> for Report {
    type Output = SubReport;

    fn index(&self, index: Uid) -> &Self::Output {
        unimplemented!()
    }
}

impl IndexMut<Uid> for Report {
    fn index_mut(&mut self, index: Uid) -> &mut Self::Output {
        unimplemented!()
    }
}
