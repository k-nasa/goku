use std::sync::Arc;
use std::time::{Instant, Duration};

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::channel;
use async_std::task;
use log::{debug, info};

pub type GokuResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn attack(concurrency: usize, requests: usize, host: &str, port: u16) -> GokuResult<()> {
    let host = format!("{}:{}", host, port);

    let request = format!("GET / HTTP/1.1\nHost: {}\nUser-Agent: goku/0.0.1\n\n", host);

    let now = Instant::now();
    let (s, r) = channel(concurrency);

    let send_handler = task::spawn(async move {
        let host = Arc::<str>::from(host);
        let request = Arc::<str>::from(request);

        for _ in 0..requests {
            let host = host.to_string();
            let request = request.to_string();

            let handler = task::spawn(async move { send_request(&host, &request).await });
            s.send(handler).await;
        }
    });

    let receive_handler = task::spawn(async move {
        let mut count = 0;
        let mut all_time = Vec::new();
        let mut errors = Vec::new();

        while let Some(v) = r.recv().await {
            match v.await {
                Err(e) => {
                    errors.push(e);
                }
                Ok(duration) => {
                    count += 1;
                    all_time.push(duration);
                }
            }
        }

        errors.iter().for_each(|e| debug!("{}", e));
        println!("Complete requests: {}", count);
        println!("Failed requests:   {}", errors.iter().count());
        println!("Latency:");
        println!("    max: {:?}", all_time.iter().max().unwrap_or(&Duration::new(0,0)));
        println!("    min: {:?}", all_time.iter().min().unwrap_or(&Duration::new(0,0)));
        println!("    ave: {:?}", all_time.iter().sum::<Duration>() / count);
        println!("    ave: {:?} (mean, across all concurrent requests)", now.elapsed() / count);
    });

    task::block_on(async { async_std::future::join![send_handler, receive_handler].await });

    Ok(())
}

pub async fn send_request(host: &str, request: &str) -> Result<Duration, async_std::io::Error> {
    let now = Instant::now();

    let mut stream = TcpStream::connect(host).await?;

    stream.write(&request.as_bytes()).await?;

    Ok(now.elapsed())
}
