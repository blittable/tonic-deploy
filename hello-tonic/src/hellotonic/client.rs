use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloReply, HelloRequest};

async fn run_requests<T>(iterations: i32, f: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Fn(String),
{
    //let responses = Arc::new(Mutex::new(Vec::<HelloReply>::new()));
    //let responses = Arc::clone(&responses);

    let mut client = GreeterClient::connect("http://0.0.0.0:50003")?;

    // tokio::spawn(async move {  <- Crazy fast
        for _ in 0..1000_i32 {
            let request = tonic::Request::new(HelloRequest {
                name: "world".into(),
                iteration: 1,
            });

            let resp = match client.say_hello(request).await {
                Ok(resp) => println!("{:?}", resp.into_inner().message),
                Err(e) => {
                    println!("Errant response; err = {:?}", e);
                }
            };
        }
    //});

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting greeter client");

    let iterations: i32 = 1000;
    let start = Instant::now();

    let output = move |x| println!("message: {:?}", x);

    let result = run_requests(iterations, output).await;

    let duration = start.elapsed();

    println!(
        "Time elapsed for {:?} requests was: {:?}",
        iterations, duration
    );

    Ok(())
}
