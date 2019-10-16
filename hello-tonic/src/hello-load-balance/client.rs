use log::info;
use tonic::transport::Channel;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Starting HelloTonic Greeter Client");

    let endpoints = ["http://[::1]:50003", "http://[::1]:50004"]
        .into_iter()
        .map(|a| Channel::from_static(a));


    let channel = Channel::balance_list(endpoints);

    let mut client = GreeterClient::new(channel);

    for i in 0..1000 {
    let request = tonic::Request::new(HelloRequest {
        name: "hello".into(),
        iteration: 0,
    });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response);
    }

    Ok(())
}
