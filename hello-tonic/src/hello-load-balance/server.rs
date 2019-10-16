pub mod hellotonic {
    tonic::include_proto!("hellotonic");
}

use std::{collections::VecDeque, net::SocketAddr};
use tonic::{transport::Server, Request, Response, Status };
use tokio::sync::mpsc;

use hellotonic::{
    HelloReply, HelloRequest,
};

#[derive(Debug)]
pub struct MyGreeter {
    addr: SocketAddr,
}

#[tonic::async_trait]
impl hellotonic::server::Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request 5-6: {:?}", request);

        let reply = hellotonic::HelloReply {
            message: "Zomg, it works!".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addrs = ["192.168.1.14:50005", "192.168.1.14:50006"];
    //let addrs = ["192.168.1.14:50003", "192.168.1.14:50004", "192.168.1.14:50005", "192.168.1.14:50006"];

    let (tx, mut rx) = mpsc::unbounded_channel();

    for addr in &addrs {
        let addr = addr.parse()?;
        let mut tx = tx.clone();

        let server = MyGreeter { addr };
        let serve = Server::builder().serve(addr, hellotonic::server::GreeterServer::new(server));

        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.try_send(()).unwrap();
        });
    }

    rx.recv().await;

    Ok(())
}
