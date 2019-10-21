use futures::future;
use futures::future::FutureExt;
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use log::info;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex};
use tonic::Response;

pub mod hello_tonic {
    tonic::include_proto!("hellotonic");
}

use hello_tonic::{client::GreeterClient, HelloReply, HelloRequest};


async fn bench() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let iterations = 5000;

    let client = GreeterClient::connect("http://127.0.0.1:50003")?;

    let mut futures = FuturesUnordered::new();

    for i in 0..iterations {
        let request = tonic::Request::new(HelloRequest {
            name: "world".into(),
            iteration: i,
        });

        let mut client = client.clone();
        futures.push(async move { client.say_hello(request).await });
    }

    while let Some(res) = futures.next().await {
        match res {
            Ok(resp) => println!("{:?}", resp.into_inner().message),
            Err(e) => {
                println!("Errant response; err = {:?}", e);
            }
        }
    }

    let duration = start.elapsed();

    println!(
        "Time elapsed for {:?} requests was: {:?}",
        iterations, duration
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    bench().await?;
    Ok(())
}
