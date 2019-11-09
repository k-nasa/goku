use std::sync::Arc;
use std::time::Instant;

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::channel;
use async_std::task;
use log::{debug, info};

pub type GokuResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn attack(concurrency: usize, requests: usize, host: &str, port: u16) -> GokuResult<()> {
    let host = format!("{}:{}", host, port);

    let request = format!(
        "GET / HTTP/1.1\nHost: {}\nUser-Agent: goku/0.0.1\n\n",
        host
    );

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
        while let Some(v) = r.recv().await {
            match v.await {
                Err(e) => {
                    debug!("{}", e);
                }
                Ok(_) => count += 1,
            }
        }
        info!("count: {}", count);
    });

    task::block_on(async { async_std::future::join![send_handler, receive_handler].await });

    info!("duration: {:?}", now.elapsed());

    Ok(())
}

pub async fn send_request(host: &str, request: &str) -> Result<(), async_std::io::Error> {
    let mut stream = TcpStream::connect(host).await?;

    stream.write(&request.as_bytes()).await?;

    Ok(())
}
