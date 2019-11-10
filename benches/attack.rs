#![feature(test)]

extern crate test;

use goku::attack;
use goku::send_request;
use test::Bencher;

#[bench]
fn attack_bench(b: &mut Bencher) {
    b.iter(|| {
        attack(1, 1, &"127.0.0.1", 8080).unwrap();
    });
}

#[bench]
fn send_request_bench(b: &mut Bencher) {
    let host = format!("{}:{}", "127.0.0.1", "8080");

    let request = format!("GET / HTTP/1.1\nHost: {}\nUser-Agent: goku/0.0.1\n\n", host);
    b.iter(|| {
        async_std::task::block_on(async {
            #![allow(unused_must_use)]
            send_request(&host, &request).await;
        })
    });
}

#[bench]
fn attack_bench_50_request_10_concurrency(b: &mut Bencher) {
    b.iter(|| {
        attack(10, 50, &"127.0.0.1", 8080).unwrap();
    });
}

#[bench]
fn attack_bench_when_50_request_1_concurrency(b: &mut Bencher) {
    b.iter(|| {
        attack(1, 50, &"127.0.0.1", 8080).unwrap();
    });
}
