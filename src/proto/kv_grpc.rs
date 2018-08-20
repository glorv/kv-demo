// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KV_SERVICE_GET: ::grpcio::Method<super::kv::GetRequest, super::kv::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.KVService/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_SERVICE_PUT: ::grpcio::Method<super::kv::KVPair, super::kv::PutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.KVService/Put",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_SERVICE_SCAN: ::grpcio::Method<super::kv::ScanRequset, super::kv::ScanResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.KVService/Scan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct KvServiceClient {
    client: ::grpcio::Client,
}

impl KvServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KvServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::kv::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::GetResponse> {
        self.client.unary_call(&METHOD_KV_SERVICE_GET, req, opt)
    }

    pub fn get(&self, req: &super::kv::GetRequest) -> ::grpcio::Result<super::kv::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::kv::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::GetResponse>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::kv::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_opt(&self, req: &super::kv::KVPair, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::PutResponse> {
        self.client.unary_call(&METHOD_KV_SERVICE_PUT, req, opt)
    }

    pub fn put(&self, req: &super::kv::KVPair) -> ::grpcio::Result<super::kv::PutResponse> {
        self.put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_async_opt(&self, req: &super::kv::KVPair, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::PutResponse>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_PUT, req, opt)
    }

    pub fn put_async(&self, req: &super::kv::KVPair) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::PutResponse>> {
        self.put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_opt(&self, req: &super::kv::ScanRequset, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::ScanResponse> {
        self.client.unary_call(&METHOD_KV_SERVICE_SCAN, req, opt)
    }

    pub fn scan(&self, req: &super::kv::ScanRequset) -> ::grpcio::Result<super::kv::ScanResponse> {
        self.scan_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_async_opt(&self, req: &super::kv::ScanRequset, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::ScanResponse>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_SCAN, req, opt)
    }

    pub fn scan_async(&self, req: &super::kv::ScanRequset) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::ScanResponse>> {
        self.scan_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait KvService {
    fn get(&self, ctx: ::grpcio::RpcContext, req: super::kv::GetRequest, sink: ::grpcio::UnarySink<super::kv::GetResponse>);
    fn put(&self, ctx: ::grpcio::RpcContext, req: super::kv::KVPair, sink: ::grpcio::UnarySink<super::kv::PutResponse>);
    fn scan(&self, ctx: ::grpcio::RpcContext, req: super::kv::ScanRequset, sink: ::grpcio::UnarySink<super::kv::ScanResponse>);
}

pub fn create_kv_service<S: KvService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_SERVICE_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_SERVICE_PUT, move |ctx, req, resp| {
        instance.put(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_SERVICE_SCAN, move |ctx, req, resp| {
        instance.scan(ctx, req, resp)
    });
    builder.build()
}
