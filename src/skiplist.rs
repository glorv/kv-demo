use std::borrow::Borrow;
use std::hash::Hash;
use std::cmp::{Eq, Ordering, PartialOrd};
use std::fmt::Debug;
use std::mem;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::fs::File;

use bincode::{serialize, deserialize};
use iter::KVIterator;
use io::{KVDataOutput, KVDataInput, MAGIC_NUMBER, KVAction};
use fs::{FsDataInput, FsDataOutput};
use serde::{Serialize, Deserialize};

use rand::prelude::random;
use errors::*;

const MAX_HEIGHT: u8 = 20;
const HEIGHT_INCR: u32 = u32::max_value() / 3;
const ALOG_CURRENT: i32 = 1;


#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K: Clone + Hash + Eq, V: Debug> Entry<K, V> {
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Entry {
            key, value
        }
    }
}

#[derive(Debug)]
pub struct Node<K, V> {
    index: usize,
    entry: Option<Entry<K, V>>,
    height: u8,
    pub tower: Vec<u32>,
}

impl<K: Clone + Hash + Eq + Debug, V: Debug> Node<K, V> {
    pub fn new(key: K, value: V, height: u8) -> Node<K, V> {
        Self::build(Some(Entry::new(key, value)), height)
    }

    pub fn empty(height: u8) -> Node<K, V> {
        Self::build(None, height)
    }

    fn build(entry: Option<Entry<K, V>>, height: u8) -> Node<K, V> {
        let mut tower = Vec::with_capacity(height as usize);
        tower.resize(height as usize, 0);
        Node {
            index: 0,
            entry,
            height,
            tower,
        }
    }
}

pub struct Table<K, V> {
    height: u8,
    pub node_table: Vec<Node<K, V>>,
    free_indexes: Vec<usize>,
    // min_key_index is equal to `self.node_table[0].tower[0]`
    max_key_index: usize,
    output: Option<Box<KVDataOutput>>,
}

fn random_height() -> u8 {
    let mut h = 1u8;
    while h < MAX_HEIGHT as u8 && random::<u32>() < HEIGHT_INCR {
        h += 1
    }
    h
}

impl<K, V> Table<K, V> where K: Clone + Hash + Ord + Debug + Serialize, V: Debug + Serialize {
    pub fn with_capacity(size: usize) -> Table<K, V> {
        let head = Node::empty(MAX_HEIGHT);
        let mut node_table = Vec::with_capacity(size);
        node_table.push(head);
        Table {
            height: 1,
            node_table,
            free_indexes: Vec::new(),
            max_key_index: 0,
            output: None,
        }
    }

    pub fn open<T: AsRef<Path>>(path: &T, capacity: usize) -> Result<Table> {
        if Path::is_file(path) {
            let mut table = Self::read_from_file(path)?;
            let output = FsDataOutput::open_exist(path)?;
            table.output = Some(output);
            Ok(table)
        } else {
            let mut table = Self::with_capacity(capacity);
            let output = FsDataOutput::open_new(path)?;
            table.add_output_header(output, capacity)?;
            table.output = Some(output);
            Ok(table)
        }
    }

    fn add_output_header(&self, output: &mut KVDataOutput, capacity: usize) -> Result<()> {
        output.write_int(MAGIC_NUMBER)?;
        output.write_int(ALOG_CURRENT)?;
        output.write_vint(capacity as i32)
    }

    fn read_from_file(path: &T) -> Result<Table<K, V>> {
        assert!(Path::is_file(path));
        let mut reader = FsDataInput::open_input(path)?;

        let magic = reader.read_int()?;
        if magic != MAGIC_NUMBER {
            bail!("invalid magic number {}", magic);
        }
        let version = reader.read_int()?;
        if version != 1 {
            bail!("unknown action log version {}", version);
        }

        let capacity = reader.read_vint()?;
        if capasity <= 0 {
            bail!("invalid table capacity '{}'", capacity);
        }
        let mut table = Self::with_capacity(capacity as usize);
        for _ in 0..capacity {
            match reader.read_kv_action()? {
                KVAction::Put(ref key_bytes, ref value_bytes) => {
                    let key: K = deserialize(key_bytes)?;
                    let value: V = deserialize(value_bytes)?;
                    table.put(key, value);
                }
                KVAction::Remove(ref key_bytes) => {
                    let key: K = deserialize(key_bytes)?;
                    table.remove(&K);
                }
            }
        }
        return Ok(table)
    }

    pub fn can_put(&self) -> bool {
        self.node_table.len() < self.node_table.capacity() || !self.free_indexes.is_empty()
    }

    fn is_valid_index(&self, i: u32) -> bool {
        0 < i && i < self.node_table.capacity() as u32
    }

    fn get_next_index(&self, before: usize, level: usize) -> usize {
        debug_assert!(before < self.node_table.len());
        self.node_table[before].tower[level] as usize
    }

    fn find_splice_for_level<Q: ?Sized + Ord>(
        &self,
        key: &Q,
        before: usize,
        level: usize,
        allow_equal: bool,
    ) -> (usize, usize) where K : Borrow<Q> {
        let mut before = before;
        loop {
            let next = self.get_next_index(before, level);
            if next == 0 {
                return (before, next)
            }
            let ord = key.cmp(&self.node_table[next].entry.as_ref().unwrap().key.borrow());
            if ord == Ordering::Equal {
                if allow_equal {
                    return (next, next);
                } else {
                    return (before, self.node_table[next].tower[level] as usize);
                }
            } else if ord == Ordering::Less {
                return (before, next)
            } else {
                before = next;
            }
        }
    }

    fn reserve_index(&mut self) -> usize {
        if let Some(idx) = self.free_indexes.pop() {
            idx
        } else {
            self.node_table.len()
        }
    }

    pub fn get<Q: ?Sized + Ord>(&self, key: &Q) -> Option<&V>
    where K: Borrow<Q> {
        let idx = self.find_near_index(key, false, true);
        if idx > 0 {
            if let Some(ref entry) = self.node_table[idx].entry {
                if (&entry.key).borrow() == key {
                    return Some(&entry.value);
                }
            }
        }
        None
    }

    fn write_put_alog(&mut self, key: &K, value: &V) -> Result<()> {
        if let Some(ref mut out) = self.output {
            let key_bytes = serialize(key)?;
            let value_bytes = serialize(value)?;
            out.write_kv_action(&KVAction::Put(key_bytes, value_bytes))
        }
        Ok(())
    }

    pub fn put(&mut self, key: K, value: V) -> Result<()> {
        let mut prev = Vec::with_capacity((MAX_HEIGHT + 1) as usize);
        let mut next = Vec::with_capacity((MAX_HEIGHT + 1) as usize);

        prev.resize((MAX_HEIGHT + 1) as usize, 0);
        next.resize((MAX_HEIGHT + 1) as usize, 0);
        for i in 0..self.height {
            let idx = (self.height - 1 - i) as usize;
            let (p, n) = self.find_splice_for_level(&key, prev[idx+1], idx, true);
            prev[idx] = p;
            next[idx] = n;
            if prev[idx] > 0 && prev[idx] == next[idx] {
                self.write_put_alog(&key, &value)?;
                self.node_table[prev[idx]].entry.as_mut().unwrap().value = value;

                return Ok(());
            }
        }

        assert!(self.node_table.len() < self.node_table.capacity() ||
            !self.free_indexes.is_empty());

        let height = random_height();
        self.write_put_alog(&key, &value)?;
        let mut node = Node::new(key, value, height);
        let node_index = self.reserve_index();
        node.index = node_index;
        let orig_height = self.height;
        if self.height < height {
            self.height = height;
        }

        for i in 0..height as usize {
            if i > orig_height as usize {
                let (p, n) = self.find_splice_for_level(&node.entry.as_ref().unwrap().key,
                                                        0, i, true);
                prev[i] = p;
                next[i] = n;
            }
            assert!(prev[i] == 0 || prev[i] != next[i]);
            let next_idx = if next[i] == 0 {
                0
            } else {
                self.node_table[next[i]].index
            };
            node.tower[i] =next_idx as u32;
            self.node_table[prev[i]].tower[i] = node_index as u32;
        }
        // this is the max key now
        if node.tower[0] == 0 {
            self.max_key_index = node_index;
        }
        if self.node_table.len() == node.index {
            self.node_table.push(node);
        } else {
            self.node_table[node_index] = node;
        }
        Ok(())
    }

    fn write_remove_alog<Q: ?Sized + Ord + Serialize>(&mut self, key: &Q) -> Result<()> where K: Borrow<Q> {
        if let Some(ref mut out) = self.output {
            let key_bytes = serialize(key)?;
            out.write_kv_action(&KVAction::Remove(key_bytes))
        }
        Ok(())
    }

    pub fn remove<Q: ?Sized + Ord + Serialize>(&mut self, key: &Q) -> Result<Option<V>> where K: Borrow<Q> {
        let idx = self.find_near_index(key, false, true);
        if idx > 0 && (&self.node_table[idx].entry.as_ref().unwrap().key).borrow() == key {
            self.write_remove_alog(key);
            let height = self.node_table[idx].height;
            let mut prev = 0usize;
            for j in 0..height {
                let i = (height - j - 1) as usize;
                let (p, n) = self.find_splice_for_level(key, prev, i, false);
                self.node_table[p].tower[i] = n as u32;
                prev = p;
            }
            // after delete max key, the new max will be the level 0 prev
            if self.node_table[idx].tower[0] == 0 {
                self.max_key_index = prev;
            }
            let entry = mem::replace(&mut self.node_table[idx].entry, None);
            self.free_indexes.push(idx);
            return Ok(Some(entry.unwrap().value));
        }
        Ok(None)
    }

    fn find_near_index<Q: ?Sized + Ord>(&self, key: &Q, less: bool, allow_equal: bool) -> usize
    where K: Borrow<Q> {
        let mut x = 0usize;
        let mut level = self.height - 1;
        loop {
            let next_index = self.get_next_index(x, level as usize);
            if next_index == 0 {
                if level > 0 {
                    level -= 1;
                    continue;
                }
                if !less {
                    return 0;
                }
                if x == 0 {
                    return 0;
                }
                return x;
            }
            let next_key = &self.node_table[next_index].entry.as_ref().unwrap().key;
            match key.cmp(next_key.borrow()) {
                Ordering::Greater => {
                    x = next_index;
                    continue;
                }
                Ordering::Equal => {
                    if allow_equal {
                        return next_index;
                    }
                    if !less {
                        return self.get_next_index(next_index, 0);
                    }
                    if level > 0 {
                        level -= 1;
                        continue;
                    }
                    if x == 0 {
                        return 0;
                    }
                    return x;
                }
                Ordering::Less => {
                    if level > 0 {
                        level -= 1;
                        continue;
                    }
                    if !less {
                        return next_index;
                    }
                    if x == 0 {
                        return 0;
                    }
                    return x;
                }
            }
        }
    }

    pub fn max_key(&self) -> Option<&K> {
        if self.max_key_index == 0 {
            None
        } else {
            Some(&self.node_table[self.max_key_index].entry.as_ref().unwrap().key)
        }
    }

    pub fn min_key(&self) -> Option<&K> {
        let idx = self.node_table[0].tower[0] as usize;
        if idx == 0 {
            None
        } else {
            Some(&self.node_table[idx].entry.as_ref().unwrap().key)
        }
    }

    pub fn iter(&self) -> SkipListIterator<K, V, &Table<K, V>> {
        SkipListIterator {
            skip_list: self,
            node_idx: 0,
        }
    }
}

pub struct SkipListIterator<K, V, T: Deref<Target=Table<K, V>>> {
    skip_list: T,
    node_idx: usize
}

impl<K, V, T> SkipListIterator<K, V, T> where K: Clone + Hash + Ord + Debug, V: Debug, T: Deref<Target=Table<K, V>> {
    pub fn new(t: T) -> SkipListIterator<K, V, T> {
        SkipListIterator {
            skip_list: t,
            node_idx: 0,
        }
    }
}

impl<K, V, T> Eq for SkipListIterator<K, V, T> where K: Clone + Hash + Ord + Debug, V: Debug, T: Deref<Target=Table<K, V>> {
}

impl<K, V, T> PartialEq for SkipListIterator<K, V, T> where K: Clone + Hash + Ord + Debug, V: Debug, T: Deref<Target=Table<K, V>> {
    fn eq(&self, other: &Self) -> bool {
        self.valid() && other.valid() && &self.skip_list.node_table[self.node_idx].entry.as_ref().unwrap().key
            == &other.skip_list.node_table[other.node_idx].entry.as_ref().unwrap().key
    }
}

impl<K, V, T> Ord for SkipListIterator<K, V, T> where K: Clone + Hash + Ord + Debug, V: Debug, T: Deref<Target=Table<K, V>> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<K, V, T> PartialOrd for SkipListIterator<K, V, T> where K: Clone + Hash + Ord + Debug, V: Debug, T: Deref<Target=Table<K, V>> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.valid() {
            if !other.valid() {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Less)
            }
        } else if !other.valid() {
            Some(Ordering::Greater)
        } else {
            Some((&other.skip_list.node_table[other.node_idx].entry.as_ref().unwrap().key).cmp(
                &self.skip_list.node_table[self.node_idx].entry.as_ref().unwrap().key))
        }
    }
}

impl<K, V, T> KVIterator<K, V> for SkipListIterator<K, V, T> where K: Clone + Hash + Ord + Debug, V: Debug, T: Deref<Target=Table<K, V>> {
    fn valid(&self) -> bool {
        unsafe {
            self.node_idx < self.skip_list.node_table.len()
        }
    }

    fn key(&self) -> &K {
        debug_assert!(self.valid());
        unsafe {
            &self.skip_list.node_table[self.node_idx].entry.as_ref().unwrap().key
        }
    }

    fn value(&self) -> &V {
        debug_assert!(self.valid());
        unsafe {
            &self.skip_list.node_table[self.node_idx].entry.as_ref().unwrap().value
        }
    }

    fn next(&mut self) {
        debug_assert!(self.valid());
        let node_index = self.skip_list.get_next_index(self.node_idx, 0);
        if node_index > 0 {
            self.node_idx = node_index;
        } else {
            self.node_idx = usize::max_value();
        }
    }

//    fn prev(&mut self) {
//        debug_assert!(self.has_next());
//        unsafe {
//            let node_index = (*self.skip_list).find_near_index(self.key(), true, false);
//            if node_index > 0 {
//                self.node_idx = node_index
//            } else {
//                self.node_idx = usize::max_value();
//            }
//        }
//    }

    fn advance<Q: ?Sized + Ord>(&mut self, key: &Q) where K: Borrow<Q> {
        debug_assert!(self.valid());
        unsafe {
            let node_index = self.skip_list.find_near_index(key, false, true);
            if node_index > 0 {
                self.node_idx = node_index
            } else {
                self.node_idx = usize::max_value();
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use iter::tests::rand_int_array;

    #[test]
    fn test_skip_list() {
        let mut table: Table<String, i32> = Table::with_capacity(65536);
        for i in 0..100 {
            table.put(i.to_string(), i);
        }
        for i in 0..100 {
            let v = table.get(&i.to_string());
            assert_eq!(v, Some(&i))
        }

        table.put("50".to_string(), 100);
        assert_eq!(table.get("50"), Some(&100));

        table.put("50".to_string(), 50);
        assert_eq!(table.get("50"), Some(&50));

        for i in 0..100 {
            assert_eq!(table.remove(&i.to_string()), Some(i));
        }

        let mut table: Table<u32, String> = Table::with_capacity(65536);
        let mut v = rand_int_array();
        for i in &v {
            table.put(*i, i.to_string());
        }

        for i in &v {
            assert_eq!(table.get(i), Some(&i.to_string()));
        }

        loop {
            if v.is_empty() {
                break;
            }

            let mut copy = v.clone();
            copy.sort();
            let mut idx = 0usize;
            {
                let mut iter = table.iter();
                loop {
                    iter.next();
                    if !iter.valid() {
                        break;
                    }
                    assert_eq!(iter.key(), &copy[idx]);
                    idx += 1;
                }
            }
            let i = v.pop().unwrap();
            table.remove(&i);
        }
    }

    #[test]
    fn test_skip_list_max_idx() {
        let mut table: Table<u32, String> = Table::with_capacity(65536);
        for i in 0..100 {
            table.put(i, i.to_string());
            assert_eq!(table.max_key(), Some(&i));
        }
        assert_eq!(table.min_key(), Some(&0));

        table.remove(&99);
        assert_eq!(table.max_key(), Some(&98));
        assert_eq!(table.min_key(), Some(&0));

        table.remove(&50);
        assert_eq!(table.max_key(), Some(&98));
        assert_eq!(table.min_key(), Some(&0));

        table.remove(&0);
        assert_eq!(table.max_key(), Some(&98));
        assert_eq!(table.min_key(), Some(&1));
    }
}