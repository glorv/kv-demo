use std::borrow::Borrow;
use std::hash::Hash;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::collections::BinaryHeap;
use std::sync::{RwLock, RwLockReadGuard, Arc};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::sync::atomic::AtomicUsize;

use skiplist::{Table, SkipListIterator};
use iter::KVIterator;
use fs::walk_dir_entries;

const ALOG_POSTFIX: &str = ".alog";

pub trait Storage<K, V> {
    type Iter: KVIterator<K, V>;
    fn put(&mut self, key: K, value: V);
    fn get<Q: ?Sized + Ord>(&self, key: &Q) -> Option<V> where K: Borrow<Q>;
    fn remove<Q: ?Sized + Ord>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>;
    fn iter(&self) -> Self::Iter;
}

struct Config {
    pub table_capacity: usize,
    pub merge_table_count: u32
}

struct Database<K, V> {
    tables: Arc<RwLock<Vec<Arc<RwLock<Table<K, V>>>>>>,
    config: Config,
    merging_tables_idx: Vec<usize>,
    version_counter: AtomicUsize,
}

impl<K: Clone + Hash + Ord + Debug, V: Debug + Clone> Database<K, V> {
    pub fn new(config: Config) -> Database<K, V> {
        let mut tables = Vec::new();
        let table = Arc::new(RwLock::new(Table::with_capacity(config.table_capacity)));
        tables.push(table);
        Database {
            tables: Arc::new(RwLock::new(tables)),
            config,
            merging_tables_idx: Vec::new(),
            version_counter: AtomicUsize::new(0),
        }
    }

    pub fn open_dir<T: AsRef<Path>>(path: &T, config: Config) -> Result<Database<K, V>> {
        if !path.as_ref().is_dir() {
            bail!("Invalid dir path '{}'", path);
        }
        let mut version = 0usize;
        let mut tables = Vec::new();
        for entry in read_dir(path)? {
            if let Some(s) = entry.file_name().to_str() {
                if s.ends_with(ALOG_POSTFIX) {
                    if let Ok(v) = s.parse::<usize>(s.index(..s.len() - ALOG_POSTFIX.len())) {
                        if version > v {
                            version = v;
                        }
                        let table = Table::open(entry.path(), config.table_capacity)?;
                        tables.push(Arc::new(RwLock::new(table)));
                    }

                }
            };
        }
    }



    pub fn put(&mut self, key: K, value: V) -> Result<()> {
        if let Ok(ref tables) = self.tables.read() {
            for table in tables.iter() {
                if let Ok(t) = table.read() {
                    if t.get(&key).is_none() {
                        continue;
                    }
                }
                if let Ok(mut t) = table.write() {
                    return t.put(key, value);
                }
            }
            for table in tables.iter() {
                if let Ok(mut t) = table.write() {
                    if t.can_put() {
                        return t.put(key, value);
                    }
                }
            }
        }
        // add new table
        let mut table = Table::with_capacity(self.config.table_capacity);
        table.put(key, value)?;
        self.tables.write().unwrap().push(Arc::new(RwLock::new(table)));
        Ok(())
    }

    pub fn get<Q: ?Sized + Ord>(&self, key: &Q) -> Option<V> where K: Borrow<Q> {
        if let Ok(ref tables) = self.tables.read() {
            for table in tables.iter() {
                if let Ok(t) = table.read() {
                    if let Some(v) = t.get(key) {
                        return Some(v.clone());
                    }
                }
            }
        }
        None
    }

    pub fn remove<Q: ? Sized + Ord>(&mut self, key: &Q) -> Result<Option<V>> where K: Borrow<Q> {
        if let Ok(ref tables) = self.tables.read() {
            for table in tables.iter() {
                if let Ok(t) = table.read() {
                    if t.get(key).is_none() {
                        continue;
                    }
                }
                if let Ok(mut t) = table.write() {
                    return t.remove(key);
                }
            }
        }
        Ok(None)
    }

    pub fn read_view(&self) -> Vec<Arc<RwLock<Table<K, V>>>> {
        let mut tables = Vec::new();
        for table in self.tables.read().unwrap().iter() {
            tables.push(table.clone());
        }
        tables
    }

//    fn iter(&self) -> <Self as Storage<K, V>>::Iter {
//        let mut tables = Vec::new();
//        for table in self.tables.read().unwrap().iter() {
//            tables.push(table.clone());
//        }
//        DatabaseIterator::new(tables)
//    }
}



struct DatabaseIterator<'a, K: 'a, V: 'a> {
    sub_iters: BinaryHeap<SkipListIterator<K, V, RwLockReadGuard<'a, Table<K, V>>>>
}

impl<'a, K: 'a + Clone + Hash + Ord + Debug, V: 'a + Debug> DatabaseIterator<'a, K, V> {
    pub fn new(tables: &'a [Arc<RwLock<Table<K, V>>>]) -> DatabaseIterator<'a, K, V> {
        let mut heap = BinaryHeap::new();
        for table in tables {
            let mut iter = SkipListIterator::new(table.read().unwrap());
            iter.next();
            heap.push(iter);
        }
        DatabaseIterator {
            sub_iters: heap,
        }
    }
}

impl<'a, K: 'a + Clone + Hash + Ord + Debug, V: 'a + Debug> KVIterator<K, V> for DatabaseIterator<'a, K, V> {
    fn valid(&self) -> bool {
        self.sub_iters.peek().unwrap().valid()
    }

    fn key(&self) -> &K {
        self.sub_iters.peek().unwrap().key()
    }

    fn value(&self) -> &V {
        self.sub_iters.peek().unwrap().value()
    }

    fn next(&mut self) {
        self.sub_iters.peek_mut().unwrap().next()
    }

    fn advance<Q: ?Sized + Ord>(&mut self, key: &Q) where K: Borrow<Q> {
        loop {
            if self.key().borrow() >= key {
                break;
            }
            self.sub_iters.peek_mut().unwrap().advance(key);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use iter::tests::rand_int_array;

    #[test]
    fn test_database() {
        let config = Config{
            table_capacity: 16,
            merge_table_count: 2,
        };
        let mut db = Database::new(config);

        for i in 0..100 {
            db.put(i.to_string(), i);
        }
        for i in 0..100 {
            let v = db.get(&i.to_string());
            assert_eq!(v, Some(i))
        }

        db.put("50".to_string(), 100);
        assert_eq!(db.get("50"), Some(100));

        db.put("50".to_string(), 50);
        assert_eq!(db.get("50"), Some(50));

        for i in 0..100 {
            assert_eq!(db.remove(&i.to_string()), Some(i));
        }

        let config = Config{
            table_capacity: 16,
            merge_table_count: 2,
        };
        let mut db = Database::new(config);
        let mut v = rand_int_array();

        for i in &v {
            db.put(*i, i.to_string());
        }

        for i in &v {
            assert_eq!(db.get(i), Some(i.to_string()));
        }

        loop {
            if v.is_empty() {
                break;
            }

            let mut copy = v.clone();
            copy.sort();
            let mut idx = 0usize;
            {
                let mut tables = db.read_view();
                let mut iter = DatabaseIterator::new(&tables);
                loop {
                    if !iter.valid() {
                        break;
                    }
                    assert_eq!(iter.key(), &copy[idx]);
                    idx += 1;
                    iter.next();
                }
            }
            let i = v.pop().unwrap();
            db.remove(&i);
        }
    }

}