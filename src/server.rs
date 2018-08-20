use std::panic::catch_unwind;
use std::sync::Arc;

use futures::Future;
use grpcio::{RpcContext, RpcStatus, RpcStatusCode, UnarySink};

use codec::BytesSerializer;
use db::{Database, DatabaseIterator};
use iter::KVIterator;
use proto::kv_grpc::KvService;
use proto::kv::*;

#[derive(Clone)]
pub struct KVServer {
    db: Arc<Database<Vec<u8>, Vec<u8>>>,
}

impl KVServer {
    pub fn new(db: Database<Vec<u8>, Vec<u8>>) -> KVServer {
        KVServer {
            db: Arc::new(db),
        }
    }
}

impl KvService for KVServer {
    fn get(&self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut r = GetResponse::new();
        if let Some(v) = self.db.get(req.get_key()) {
            let mut pair = KVPair::new();
            pair.set_key(req.get_key().to_vec());
            pair.set_value(v);
            r.set_data(pair);
            r.set_exist(true);
            r.set_status(Status::OK);
        } else {
            r.set_exist(false);
        }
        let fut = sink.success(r);
        ctx.spawn(fut.map_err(move |e| info!("failed to reply to request {:?}", e)));
    }

    fn put(&self, ctx: RpcContext, req: KVPair, sink: UnarySink<PutResponse>) {
        let mut r = PutResponse::new();
        if let Err(e) = self.db.put(req.get_key().to_vec(), req.get_value().to_vec()) {
            r.set_status(Status::FAILED);
            r.set_error(format!("{}", e));
        } else {
            r.set_status(Status::OK);
        }
        let fut = sink.success(r);
        ctx.spawn(fut.map_err(move |e| info!("failed to reply to request {:?}", e)));
    }

    fn scan(&self, ctx: RpcContext, req: ScanRequset, sink: UnarySink<ScanResponse>) {
        let tables = self.db.read_view();
        let mut iter = DatabaseIterator::new(&tables);
        let key = req.get_key();
        if !key.is_empty() {
            iter.advance(key);
        }
        let mut r = ScanResponse::new();
        let mut count = 0;
        while iter.valid() {
            if req.count <= count {
                break;
            }
            if !req.can_equal && iter.key().as_slice() == key {
                iter.next();
            }
            let mut pair = KVPair::new();
            pair.set_key(iter.key().clone());
            pair.set_value(iter.value().clone());
            r.data.push(pair);
        }
        r.set_status(Status::OK);
        let fut = sink.success(r);
        ctx.spawn(fut.map_err(move |e| info!("failed to reply to request {:?}", e)));
    }
}
