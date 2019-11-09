use std::time::Instant;

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::channel;
use async_std::task;
use env_logger as logger;
use log::{debug, info};

// TODO: change to command line arguments
const MAX_CONNECTIONS: usize = 100;
const REQUEST_AMMOUNT: usize = 10_000;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    logger::init();

    let (s, r) = channel(MAX_CONNECTIONS);

    let mut count = 0;

    let now = Instant::now();
    task::spawn(async move {
        for _ in 0..REQUEST_AMMOUNT {
            let handler = task::spawn(async { send_request().await });
            s.send(handler).await;
        }
    });

    task::block_on(async {
        while let Some(v) = r.recv().await {
            match v.await {
                Err(e) => {
                    debug!("{}", e);
                }
                Ok(_) => count += 1,
            }
        }
    });

    info!("count: {}", count);
    info!("duration: {:?}", now.elapsed());

    Ok(())
}

async fn send_request() -> Result<(), async_std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    stream.write_all(b"GET / HTTP/1.1\n").await?;
    stream.write_all(b"Host: localhost:8080\n").await?;
    stream.write_all(b"User-Agent: goku/0.0.1\n").await?;
    stream.write_all(b"Accept: */*\n").await?;
    stream.write_all(b"\n").await?;

    Ok(())
}
