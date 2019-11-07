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
                    future::timeout(Duration::from_millis(500), send_request()).await
                });
                s.send(handler).await;
            }
        });

        task::spawn(async move {
            while let Some(v) = r.recv().await {
                match v.await {
                    Err(e) => {
                        // println!("{}", e)
                    },
                    Ok(_) => count += 1,
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
    // let now = Instant::now();

    surf::get("http://localhost:8080")
        .set_header("Host", "localhost:8080")
        .set_header("User-Agent", "goku/0.0.1")
        .set_header("Accept-Encoding", "gzip")
        .set_header("Expect", "")
        .set_header("Connection", "keep-alive")
        .set_header("Transfer-Encoding", "")
        .await.unwrap()
        ;

    // println!("{:?}", now.elapsed().as_millis());
}
