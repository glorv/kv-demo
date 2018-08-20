A rust implement of in memory key-value store use lsm-like Algorithm that support `get`, `put`, `remove` and
`scan` cmd.
This demo provide a client/server that depend on [grpc-rs](https://github.com/pingcap/grpc-rs)
This KV Store Server support value of any type that implement `BytesSerializer`.

## Build
rust nightly is required for build.
```
$ cargo build --release
```
this will generate the server binary

## Usage
### start server
```
target/release/server -d path-to-data-directory -p server-port
```

### use client
```
extern crate kvdemo;

use kvdemo::client::{ConnectOptions, KVGrpcClient};
use kvdemo::codec::BytesSerializer;

let options = ConnectOptions::new("127.0.0.1:8000".into(), 10);
let client = KVGrpcClient::open(options);

let key = "test-key".to_string();
let value: i32 = 123;
assert!(client.put(&k, &value).is_ok());

let value: i32 = client.get(&k)?;
assert_eq!(value, 123);
```
refer [example](https://github.com/glorv/kv-demo/blob/master/src/bin/test_client.rs) for more example usage.

## TODO
- []implement aync merge process to reduce table number
- []implement snapshot storage persistence instead of action log


