use log::{info, warn};

use std::any::type_name;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::vec::Vec as StdVec;

use crate::{Report, Uid};

pub struct Vec<T>(StdVec<T>, Uid);

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self(StdVec::new(), Uid::new())
    }
}

// Std Vec methods
impl<T> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(StdVec::with_capacity(cap), Uid::new())
    }

    pub fn push(&mut self, item: T) {
        // let current_cap = self.0.capacity();
        if self.at_capacity() {
            warn!("{} exceeded capacity {}", self, self.0.capacity());
        }

        self.0.push(item);

        info!("{} reached capacity {}", self, self.0.capacity());
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

// Extra Vec methods
impl<T> Vec<T> {
    #[inline]
    fn at_capacity(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    pub fn set_name(&mut self, name: &str) {
        let mut report = Report::get_mut(self.1);
        report.instance_name = name.to_owned();
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.set_name(name);
        self
    }
}

impl<T> Display for Vec<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let name = &Report::get(self.1).instance_name;
        write!(fmt, "{name}: Vec<{}>", type_name::<T>())
    }
}
