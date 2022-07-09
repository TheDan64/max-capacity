use log::{info, warn};
use uuid::Uuid;

use std::any::type_name;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::vec::Vec as StdVec;

struct MetaData {
    name: String,
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            name: Uuid::new_v4().to_string(),
        }
    }
}

pub struct Vec<T>(StdVec<T>, MetaData);

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self(StdVec::new(), MetaData::default())
    }
}

// Std Vec methods
impl<T> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(StdVec::with_capacity(cap), MetaData::default())
    }

    pub fn push(&mut self, item: T) {
        if self.at_capacity() {
            warn!(
                "{} attempted to exceed capacity {}",
                self,
                self.0.capacity()
            );
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
    fn at_capacity(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    pub fn with_name(self, name: &str) -> Self {
        let md = MetaData {
            name: name.to_owned(),
            ..self.1
        };

        Self(self.0, md)
    }
}

impl<T> Display for Vec<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "Vec<{}> {}", type_name::<T>(), self.1.name)
    }
}
