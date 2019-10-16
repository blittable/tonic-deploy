use tonic::{transport::Server, Request, Response, Status};

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{
    server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[derive(Default)]
pub struct MyGreeter {
    data: String,
}

pub struct Bucket {
    data: Vec<String>
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {

        let reply = hello_tonic::HelloReply {
            message: "Hello".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50003".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        .serve(addr, GreeterServer::new(greeter))
        .await?;

    println!("Server Started");

    Ok(())
}
