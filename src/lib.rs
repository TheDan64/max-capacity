use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use sealed::Reportable;

pub mod collections;
pub mod vec;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uid(u64);

impl Uid {
    fn new() -> Self {
        static NEXT_UID: AtomicU64 = AtomicU64::new(0);

        Uid(NEXT_UID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReportEvent {
    /// Defines an objective limit in capacity that should not be passed;
    /// this might be the with_capacity method or even a shrink or reserve call.
    MaxCapacitySet(usize),
    CapacityIncrease(usize, usize),
    CapacityDecrease(usize, usize),
}

#[derive(Clone, Default, Debug)]
pub struct LineItem {
    // REVIEW: Do we want to print the uid in the report?
    // pub id: Uid,
    pub instance_name: String,
    pub events: Vec<ReportEvent>,
}

impl LineItem {
    // TODO: Use the tabled or prettytable-rs crates
    fn print(&self) {}
}

static REPORT_DATA: OnceCell<DashMap<Uid, LineItem>> = OnceCell::new();

pub struct Report;

impl Report {
    pub(crate) fn new_line_item() -> Uid {
        let id = Uid::new();
        REPORT_DATA
            .get_or_init(DashMap::new)
            .insert(id, LineItem::default());
        id
    }

    pub fn print() {
        for ref_multi in REPORT_DATA.get_or_init(DashMap::new).iter() {
            let (_id, line_item) = ref_multi.pair();
            line_item.print()
        }
    }

    pub fn get_line_item<R: Reportable>(reportable: &R) -> LineItem {
        Report::get(reportable.id()).clone()
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

mod sealed {
    use super::Uid;

    pub trait Reportable {
        fn id(&self) -> Uid;
    }
}
