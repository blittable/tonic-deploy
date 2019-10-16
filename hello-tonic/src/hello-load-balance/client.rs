use log::info;
use tonic::transport::Channel;
use std::time::Instant;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Starting HelloTonic Greeter Client");

    // let channel = Channel::balance_list(endpoints);
    // let mut client = GreeterClient::new(channel);

    let start = Instant::now();
    let iterations: i32 = 1000;

    let endpoints = ["http://192.168.1.14:50003", "http://192.168.1.14:50004"]
        .into_iter()
        .map(|a| Channel::from_static(a));

    let channel = Channel::balance_list(endpoints);
    let mut client = GreeterClient::new(channel);



    for i in 0..iterations {

        let request = tonic::Request::new(HelloRequest {
            name: "hello".into(),
            iteration: 1,
        });
        let response = client.say_hello(request).await?;

        if i % 100 == 0 {
            println!("CLIENT_RECEIVED_RESPONSE: {:?}", response);
        }
    }

    let duration = start.elapsed();
    println!(
        "Time elapsed for {:?} requests was: {:?}",
        iterations, duration
    );


    Ok(())
}
