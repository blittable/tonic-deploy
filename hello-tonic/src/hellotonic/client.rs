use log::info;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting HelloTonic Greeter Client");

    let mut client = GreeterClient::connect("http://0.0.0.0:50003")?;

    let request = tonic::Request::new(HelloRequest {
        name: "hello".into(),
        iteration: 0,
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
