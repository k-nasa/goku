use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::channel;
use async_std::task;

pub type GokuResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GokuReport {
    errors: Vec<String>,
    concurrency_level: usize,
    time_taken_test: Duration,
    complete_requests: usize,
    failed_requests: usize,
    total_transferred: usize,
    latency_max: Duration,
    latency_min: Duration,
    latency_ave: Duration,
    latency_ave_concurrency: Duration,
}

impl GokuReport {
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}

impl std::fmt::Display for GokuReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
Concurrency Level {}
Time taken for tests:   {:?}
Complete requests:      {}
Failed requests:        {}
Total transferred:      {} bytes
Latency:
  max: {:?}
  min: {:?}
  ave: {:?}
  ave: {:?} (mean, across all concurrent requests)",
            self.concurrency_level,
            self.time_taken_test,
            self.complete_requests,
            self.failed_requests,
            self.total_transferred,
            self.latency_max,
            self.latency_min,
            self.latency_ave,
            self.latency_ave_concurrency
        )
    }
}

pub fn attack(concurrency: usize, requests: usize, host: &str, port: u16) -> GokuResult<GokuReport> {
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

        let report = GokuReport {
            errors: errors.iter().map(|e| e.to_string()).collect(),
            concurrency_level: concurrency,
            time_taken_test: duration,
            complete_requests: count,
            failed_requests: errors.iter().count(),
            total_transferred: all_bytes,
            latency_max: *all_time.iter().max().unwrap_or(&Duration::new(0, 0)),
            latency_min: *all_time.iter().min().unwrap_or(&Duration::new(0, 0)),
            latency_ave: all_time.iter().sum::<Duration>() / count as u32,
            latency_ave_concurrency: duration / count as u32,

        };

        report
    });

    let (_, report) = task::block_on(async { async_std::future::join![send_handler, receive_handler].await });

    Ok(report)
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
