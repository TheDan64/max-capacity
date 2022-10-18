use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use once_cell::sync::OnceCell;

pub mod collections;
pub mod vec;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Uid(u64);

impl Uid {
    fn new() -> Self {
        static NEXT_UID: AtomicU64 = AtomicU64::new(0);

        Uid(NEXT_UID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug)]
pub(crate) enum ReportEvent {
    CapacityIncrease(usize, usize),
    CapacityDecrease(usize, usize),
}

#[derive(Default, Debug)]
pub(crate) struct LineItem {
    pub instance_name: String,
    pub events: Vec<ReportEvent>,
    /// Defines an objective limit in capacity that should not be passed;
    /// this might be the with_capacity method or even a shrink or reserve call.
    pub max_capacity: Option<usize>,
}

static REPORT_DATA: OnceCell<DashMap<Uid, LineItem>> = OnceCell::new();

pub struct Report;

impl Report {
    // REVIEW: Maybe Uid::new() should insert into Report record?
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new() -> Uid {
        let id = Uid::new();
        REPORT_DATA
            .get_or_init(DashMap::new)
            .insert(id, LineItem::default());
        id
    }

    pub fn print() {
        for ref_multi in REPORT_DATA.get_or_init(DashMap::new).iter() {
            let (id, events) = ref_multi.pair();
            // TODO: Use the tabled or prettytable-rs crates
            dbg!(id, events);
        }
    }

    pub(crate) fn get(id: Uid) -> Ref<'static, Uid, LineItem> {
        REPORT_DATA
            .get_or_init(DashMap::new)
            .entry(id)
            .or_default()
            .downgrade()
    }

    pub(crate) fn get_mut(id: Uid) -> RefMut<'static, Uid, LineItem> {
        REPORT_DATA.get_or_init(DashMap::new).entry(id).or_default()
    }
}
