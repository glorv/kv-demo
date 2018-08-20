use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use errors::Result;
use proto::kv_grpc::KvServiceClient;
use proto::kv::*;

pub struct ConnectOptions {
    pub addr: String,   //host:port
    pub batch_size: usize
}

impl Default for ConnectOptions {
    fn default() -> Self {
        ConnectOptions::new("127.0.0.1:8000".into(), 10)
    }
}

impl ConnectOptions {
    pub fn new(addr: String, batch_size: usize) -> ConnectOptions {
        ConnectOptions {
            addr, batch_size,
        }
    }
}

pub struct KVGrpcClient {
    options: ConnectOptions,
    client: KvServiceClient,
}

impl KVGrpcClient {
    pub fn open(options: ConnectOptions) -> Self {
        let channel = ChannelBuilder::new(Arc::new(EnvBuilder::new().build())).connect(&options.addr);
        let client = KvServiceClient::new(channel);
        KVGrpcClient {
            options,
            client,
        }
    }

    pub fn put(&self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let mut pair = KVPair::new();
        pair.set_key(key);
        pair.set_value(value);
        let res = self.client.put(&pair)?;
        if res.get_status() == Status::OK {
            Ok(())
        } else {
            bail!("put failed, err: {}", res.get_error())
        }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let mut req = GetRequest::new();
        req.set_key(key.to_vec());
        let res = self.client.get(&req)?;
        if res.get_status() == Status::OK {
            if res.get_exist() {
                Ok(Some(res.get_data().get_value().to_vec()))
            } else {
                Ok(None)
            }
        } else {
            bail!("get failed")
        }
    }

    pub fn scan<'a>(&'a self, key: Option<Vec<u8>>) -> Result<KVIterator<'a>> {
        let mut req = ScanRequset::new();
        req.set_key(key.unwrap_or(Vec::new()));
        req.set_can_equal(true);
        req.set_count(self.options.batch_size as i32);
        let mut res = self.client.scan(&req)?;
        let has_more = res.get_data().len() >= self.options.batch_size;
        if res.get_status() != Status::OK{
            bail!("scan failed by error: {}", res.get_error())
        } else {
            let mut buf_data = res.take_data().to_vec();
            buf_data.reverse();
            if has_more {
                // set next request info
                req.set_key(buf_data[0].get_key().to_vec());
                req.set_can_equal(false);
            }
            Ok(KVIterator {
                client: self,
                scan_request: req,
                buf_data,
                has_more,
            })
        }
    }

//    pub fn remove(&self, key: Vec<u8>) -> Result<()> {
//        self.client.delete
//    }
}

pub struct KVIterator<'a> {
    client: &'a KVGrpcClient,
    scan_request: ScanRequset,
    buf_data: Vec<KVPair>,
    has_more: bool,
}

impl<'a> KVIterator<'a> {
    fn fetch(&mut self) -> Result<()> {
        let mut res = self.client.client.scan(&self.scan_request)?;
        self.has_more = res.get_data().len() >= self.client.options.batch_size;
        let mut buf_data = res.take_data().to_vec();
        buf_data.reverse();
        if self.has_more {
            self.scan_request.set_key(buf_data[0].get_key().to_vec());
            self.scan_request.set_can_equal(false);
        }
        self.buf_data = buf_data;
        Ok(())
    }
}

impl<'a> Iterator for KVIterator<'a> {
    type Item = KVPair;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let kv = self.buf_data.pop();
        if kv.is_none() {
            if !self.has_more {
                return None;
            }
            if let Err(e) = self.fetch() {
                warn!("scan fetch failed! error: {:?}", e);
                return None;
            }
            self.next()
        } else {
            kv
        }
    }
}
