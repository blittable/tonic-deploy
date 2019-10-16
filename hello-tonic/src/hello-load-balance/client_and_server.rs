use std::time::Instant;
use tracing::info;
use tracing_attributes::instrument;

use tonic::{transport::Server, Request, Response, Status};

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}


use hello_tonic::{
    server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

use hello_tonic::client::GreeterClient;

#[derive(Default)]
pub struct MyGreeter {
    _data: String,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[instrument]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let iteration = request.into_inner().iteration;

        if iteration % 100 == 0 {
            info!("SERVER_RECEIVED_REQUEST: iteration number {:?}", iteration);
        }

        let reply = hello_tonic::HelloReply {
            message: "Zomg, it works!".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HelloWorld");
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive("echo=trace".parse()?))
        .finish();

    //tracing::subscriber::set_global_default(subscriber)?;

    let addr = "[::0]:50003".parse().unwrap();
    let greeter = MyGreeter::default();

    //Spawning to prevent server loop blocking
    tokio::spawn(async move {
        Server::builder()
            .serve(addr, GreeterServer::new(greeter))
            .await;
    });


    let mut client = GreeterClient::connect("http://[::0]:50003")?;

    let start = Instant::now();
    let iterations: i32 = 1000;

    for i in 0..iterations {
        let request = tonic::Request::new(HelloRequest {
            name: "hello".into(),
            iteration: i,
        });

        let response = client.say_hello(request).await?;
        if i % 100 == 0 {
            info!("CLIENT_RECEIVED_RESPONSE: {:?}", response);
        }
    }

    let duration = start.elapsed();
    println!(
        "Time elapsed for {:?} requests was: {:?}",
        iterations, duration
    );

    Ok(())
}
