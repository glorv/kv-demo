extern crate error_chain;
extern crate kvdemo;
extern crate rand;

use rand::prelude::random;

use kvdemo::errors::Result;
use kvdemo::client::{ConnectOptions, KVGrpcClient};
use kvdemo::codec::BytesSerializer;

fn rand_int_array(count: usize) -> Vec<u32> {
    let mut v: Vec<u32> = (0..100u32).collect();
    for i in 0..100 {
        let idx = (random::<u32>() % 100) as usize;
        let tmp = v[i];
        v[i] = v[idx];
        v[idx] = tmp;
    }
    v.truncate(count);
    v
}

fn main() -> Result<()> {
    let options = ConnectOptions::new("127.0.0.1:8000".into(), 10);
    let client = KVGrpcClient::open(options);

    let values = rand_int_array(10);
    println!("test values: {:?}", &values);

    let gen_key = |s: u32| {
        let mut k = "s_".to_string();
        k.push_str(&s.to_string());
        k
    };

    for v in &values {
        let k = gen_key(*v);
        let key = k.to_bytes()?;
        let value = v.to_bytes()?;

        client.put(key, value)?;
        println!("put action! key: {}, value: {}", k, v);
    }

    for v in &values {
        let k = gen_key(*v);
        let key = (&k).to_bytes()?;

        if let Some(ref v) = client.get(&key)? {
            let value: u32 = BytesSerializer::from_bytes(v)?;
            println!("fetch key {}, result {}", k, value);
        }
    }

    let mut iter = client.scan(None)?;
    loop {
        if let Some(next) = iter.next() {
            let key: String = BytesSerializer::from_bytes(&next.key)?;
            let value: u32 = BytesSerializer::from_bytes(&next.value)?;
            println!("scan by key {}, value {}", key, value);
        } else {
            break;
        }
    }

    Ok(())
}