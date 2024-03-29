use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Result as IoResult;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use compact_str::CompactString;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use sealed::Reportable;
use tabled::{Panel, Table, Tabled};

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

impl Default for Uid {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:x}", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Tabled)]
pub enum ReportEvent {
    /// Defines an objective limit in capacity that should not be passed;
    /// this might be the with_capacity method or even a shrink or reserve call.
    MaxCapacitySet(usize),
    CapacityIncrease(usize, usize),
    CapacityDecrease(usize, usize),
}

#[derive(Clone, Default, Debug)]
pub struct LineItem {
    pub id: Uid,
    pub instance_name: CompactString,
    pub events: Vec<ReportEvent>,
}

impl LineItem {
    fn print(&self) {
        let mut table = Table::new(&self.events);
        table.with(Panel::header(format!(
            "{} - {}",
            self.id, self.instance_name
        )));

        println!("{table}");
    }
}

static REPORT_DATA: OnceCell<DashMap<Uid, LineItem>> = OnceCell::new();

pub struct Report;

impl Report {
    pub(crate) fn new_line_item() -> Uid {
        let id = Uid::new();
        REPORT_DATA.get_or_init(DashMap::new).insert(
            id,
            LineItem {
                id,
                instance_name: "Unnamed".into(),
                events: Vec::new(),
            },
        );
        id
    }

    pub fn print() {
        for ref_multi in REPORT_DATA.get_or_init(DashMap::new).iter() {
            let (_id, line_item) = ref_multi.pair();
            line_item.print()
        }
    }

    pub fn write_to_file(_path: &Path) -> IoResult<()> {
        unimplemented!()
    }

    pub fn get_line_item<R: Reportable>(reportable: &R) -> LineItem {
        // We do a clone here because we don't want to risk "leaking" a reference
        // to the caller which could block further writes if it doesn't get dropped
        Report::get(reportable.id()).clone()
    }

    /// This function (and uses of the return value) must never be made public
    /// or else it risks deadlocking
    pub(crate) fn get(id: Uid) -> Ref<'static, Uid, LineItem> {
        Report::get_mut(id).downgrade()
    }

    /// This function (and uses of the return value) must never be made public
    /// or else it risks deadlocking
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
