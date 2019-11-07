use std::time::{Duration, Instant};

use async_std::future;
use async_std::sync::channel;
use async_std::task;
use hyper::Client;

// TODO: change to command line arguments
const max_connections: usize = 20;
const request_ammount: usize = 10_000;

fn main() -> std::io::Result<()> {
    task::block_on(async {
        let (s, r) = channel(max_connections);

        let mut count = 0;

        let now = Instant::now();
        task::spawn(async move {
            for _ in 0..request_ammount {
                let handler = task::spawn(async {
                    future::timeout(Duration::from_secs(2), send_request()).await
                });
                s.send(handler).await;
            }
        });

        task::spawn(async move {
            while let Some(v) = r.recv().await {
                if v.await.is_ok() {
                    count += 1;
                }
            }

            println!("count: {}", count);
            println!("duration: {:?}", now.elapsed());
        })
        .await;
    });

    Ok(())
}

async fn send_request() {
    let now = Instant::now();

    // TODO: change to command line arguments
    let uri = "http://localhost:8080".parse().unwrap();
    let client = Client::default();

    let _res = client.get(uri).await;

    println!("{:?}", now.elapsed().as_millis());
}
