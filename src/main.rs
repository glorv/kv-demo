#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate bincode;
extern crate serde;

mod skiplist;
mod db;
mod iter;
mod fs;
mod errors;
mod io;

use skiplist::Table;
use iter::KVIterator;
use std::fmt;

fn main() {
    let mut table: Table<String, i32> = Table::with_capacity(65536);
    for i in 1..101 {
        table.put(i.to_string(), i);
    }
    for i in 1..101 {
        let v = table.get(&i.to_string());
        println!("key: {}, value: {:?}", i, v);
    }

    let mut idx = table.node_table[0].tower[0];
    while idx > 0 {
        println!("{:?}", &table.node_table[idx as usize]);
        idx = table.node_table[idx as usize].tower[0];
    }

    let mut iter = table.iter();
    loop {
        iter.next();
        if !iter.valid() {
            break;
        }
        //println!("key: {}, value: {}", iter.key(), iter.value());
    }
}
