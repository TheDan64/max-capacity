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

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ReportEvent {
    // TODO: Exceeds max capacity?
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
    pub(crate) fn new_line_item() -> Uid {
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

#[test]
fn test_basic_report() {
    let mut map = collections::HashMap::<u32, ()>::new().with_name("my_map");

    map.insert(0, ());
    map.insert(1, ());
    map.insert(2, ());
    map.insert(3, ());

    let id = map.get_line_item_id();
    let line_item = Report::get(id);

    assert_eq!(line_item.instance_name, "my_map");
    assert_eq!(line_item.events[0], ReportEvent::CapacityIncrease(0, 3));
    assert_eq!(line_item.events[1], ReportEvent::CapacityIncrease(3, 7));
    assert_eq!(line_item.events.len(), 2);

    // Drop is significant or else dashmap will deadlock when shrink_to is called
    drop(line_item);

    map.remove(&0);
    map.remove(&1);
    map.remove(&2);
    map.remove(&3);
    map.shrink_to(0);

    let line_item = Report::get(id);

    assert_eq!(line_item.events[2], ReportEvent::CapacityDecrease(7, 0));
    assert_eq!(line_item.events.len(), 3);
}

#[test]
fn test_exceeds_capacity_report() {}
