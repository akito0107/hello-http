extern crate futures;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;
extern crate num_cpus;

use std::io;

use futures::future;
use futures::{BoxFuture, Future};
use tokio_minihttp::{Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;
use std::time::Duration;
use std::thread;

struct Server;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut res = Response::new();
        res.header("Content-Type", "text/plain");
        res.body("hello world");
        future::ok(res)
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let mut serv = TcpServer::new(tokio_minihttp::Http, addr);
    serv.threads(num_cpus::get());
    serv.serve(|| Ok(Server))

}
