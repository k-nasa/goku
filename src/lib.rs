use std::sync::Arc;
use std::time::{Duration, Instant};

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
        let mut all_bytes = 0;
        let mut errors = Vec::new();

        while let Some(v) = r.recv().await {
            match v.await {
                Err(e) => {
                    errors.push(e);
                }
                Ok(result) => {
                    count += 1;
                    all_time.push(result.0);
                    all_bytes += result.1;
                }
            }
        }

        let duration = now.elapsed();

        errors.iter().for_each(|e| debug!("{}", e));
        println!("");
        println!("Concurrency Level:      {}", concurrency);
        println!("Time taken for tests:   {:?}", duration);
        println!("Complete requests:      {}", count);
        println!("Failed requests:        {}", errors.iter().count());
        println!("Total transferred:      {} bytes", all_bytes);
        println!("Latency:");
        println!(
            "    max: {:?}",
            all_time.iter().max().unwrap_or(&Duration::new(0, 0))
        );
        println!(
            "    min: {:?}",
            all_time.iter().min().unwrap_or(&Duration::new(0, 0))
        );
        println!("    ave: {:?}", all_time.iter().sum::<Duration>() / count);
        println!(
            "    ave: {:?} (mean, across all concurrent requests)",
            duration / count
        );
    });

    task::block_on(async { async_std::future::join![send_handler, receive_handler].await });

    Ok(())
}

type ByteSize = usize;
pub async fn send_request(
    host: &str,
    request: &str,
) -> Result<(Duration, ByteSize), async_std::io::Error> {
    let now = Instant::now();

    let mut stream = TcpStream::connect(host).await?;

    stream.write(&request.as_bytes()).await?;

    let mut buffer = vec![0u8; 102400];
    let n = stream.read(&mut buffer).await?;
    // let res = buffer.iter().filter(|s| **s != 0).map(|&s| s as char).collect::<String>();
    // println!("{}", n);

    Ok((now.elapsed(), n))
}
