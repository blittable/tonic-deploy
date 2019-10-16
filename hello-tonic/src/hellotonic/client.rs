use log::info;
use std::time::Instant;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting HelloTonic Greeter Client");

    let mut client = GreeterClient::connect("http://0.0.0.0:50003")?;

    let start = Instant::now();
    let iterations: i16 = 1000;

    for i in 0..iterations {

        let request = tonic::Request::new(HelloRequest {
            name: "world".into(),
            iteration: 1,
        });

        client.say_hello(request).await;
    }

    let duration = start.elapsed();
    println!(
        "Time elapsed for {:?} requests was: {:?}",
        iterations, duration
    );

    Ok(())
}
