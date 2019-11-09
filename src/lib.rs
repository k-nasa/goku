use std::time::Instant;

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::channel;
use async_std::task;
use env_logger as logger;
use log::{debug, info};

// TODO: change to command line arguments
const MAX_CONNECTIONS: usize = 10;
const REQUEST_AMMOUNT: usize = 10_000;

pub type GokuResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn attack() -> GokuResult<()> {
    std::env::set_var("RUST_LOG", "debug");
    logger::init();

    let (s, r) = channel(MAX_CONNECTIONS);

    let now = Instant::now();
    let send_handler = task::spawn(async move {
        for _ in 0..REQUEST_AMMOUNT {
            let handler = task::spawn(async { send_request().await });
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

async fn send_request() -> Result<(), async_std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    const HTTP_REQUEST: &'static [u8] =
        b"GET / HTTP/1.1\nHost: localhost:8080\nUser-Agent: goku/0.0.1\n\n";

    stream.write_all(HTTP_REQUEST).await?;

    Ok(())
}
