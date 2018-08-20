#![feature(extern_prelude)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate protobuf;
extern crate rand;
extern crate serde;

mod codec;
mod db;
mod errors;
mod fs;
mod io;
mod iter;
mod proto;
mod server;
mod skiplist;

use std::fmt;
use std::sync::Arc;
use std::thread;
use std::io::{self as stdio, Read};

use clap::{App, Arg};
use futures::Future;
use futures::sync::oneshot;
use grpcio::{ChannelBuilder, Environment, ServerBuilder};

use errors::Result;
use db::{Config, Database};
use iter::KVIterator;
use skiplist::Table;
use server::KVServer;

struct ServerConfig {
    db_config: Config,
    port: Option<u16>,
}

fn parse_cmdline_flags() -> Result<ServerConfig> {
    let matches = App::new("KV Demo Store Server")
        .version("0.0.1")
        .author("glorv")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .help("data directory")
                .required(false)
                .takes_value(true),
        ).arg(
        Arg::with_name("port")
            .short("p")
            .long("port")
            .help("server bind port")
            .required(false)
            .takes_value(true),
    ).get_matches();

    let data_path = matches.value_of("config").map(|s| s.to_string());
    let port = {
        if let Some(p) = matches.value_of("port") {
            Some(p.parse::<u16>()?)
        } else {
            None
        }
    };

    let db_config = Config {
        table_capacity: 65535,
        merge_table_count: 5,
        data_dir: data_path,
    };
    Ok(ServerConfig {
        db_config, port
    })
}

fn run() -> Result<()> {
    let config = parse_cmdline_flags()?;
    let database = Database::new(config.db_config)?;

    let env = Arc::new(Environment::new(num_cpus::get_physical()));
    let channel_args = ChannelBuilder::new(Arc::clone(&env))
        .keepalive_permit_without_calls(true)
        .build_args();

    let service = proto::kv_grpc::create_kv_service(KVServer::new(database));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", config.port.unwrap_or(8000))
        .channel_args(channel_args)
        .build()?;

    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = stdio::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();

    Ok(())
}

quick_main!(run);
