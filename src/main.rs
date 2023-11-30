use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use autometrics::{autometrics, prometheus_exporter};
use axum::{routing::get, Router};

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_client::GreeterClient;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
#[autometrics]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        // every other response is an error
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
        let count = CALL_COUNT.fetch_add(1, Ordering::AcqRel);

        if count % 2 == 0 {
            Err(Status::internal("error!"))
        } else {
            let reply = hello_world::HelloReply {
                message: format!("Hello {}!", request.into_inner().name),
            };
            Ok(Response::new(reply))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().count() == 1 {
        run_server().await
    } else {
        run_client().await
    }
}

async fn run_server() -> Result<()> {
    let app = Router::new().route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let rest = axum::Server::bind(&addr).serve(app.into_make_service());

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let greeter = MyGreeter::default();
    let grpc = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr);

    tokio::select! {
        res = rest => res?,
        res = grpc => res?,
    }
    Ok(())
}

async fn run_client() -> Result<()> {
    let mut client = GreeterClient::connect("http://localhost:8080").await?;

    for _ in 0..4 {
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });
        let response = client.say_hello(request).await;
        println!("RESPONSE={:?}", response);
    }
    Ok(())
}
