use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet};

pub mod vec;

pub struct HashMap<K, V>(StdHashMap<K, V>);

pub struct HashSet<K, V>(StdHashSet<K, V>);
