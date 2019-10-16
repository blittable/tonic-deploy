use log::info;
use std::time::Instant;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloRequest};

async fn run_requests<T>(iterations: i32, f: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Fn(String),
{
    for i in 0..iterations {
        let mut client = GreeterClient::connect("http://0.0.0.0:50003")?;

        tokio::spawn(async move {
            let request = tonic::Request::new(HelloRequest {
                name: "world".into(),
                iteration: 1,
            });

            let result = client.say_hello(request).await;
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting greeter client");

    let iterations: i32 = 1000;
    let start = Instant::now();

    let output = |x: String| println!("message: {:?}", x);

    run_requests(iterations, output).await;

    let duration = start.elapsed();

    println!(
        "Time elapsed for {:?} requests was: {:?}",
        iterations, duration
    );

    Ok(())
}
