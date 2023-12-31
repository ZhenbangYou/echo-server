use futures::future::join_all;
use std::thread;
use std::time::Instant;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const SOCK_ADDR: &str = "127.0.0.1:9009";
const NUM_CONNECTIONS: usize = 100;
const CONTENTS: [u8; 10] = [b'q'; 10];

async fn send_one(sock_addr: &str, contents: &[u8]) -> io::Result<()> {
    let mut s = TcpStream::connect(sock_addr).await?;
    let mut in_buf = [0; CONTENTS.len()];
    s.write(contents).await?;
    let _cnt = s.read(&mut in_buf).await?;
    Ok(())
}

fn send(sock_addr: &str, contents: &[u8]) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(join_all(
        (0..NUM_CONNECTIONS).map(|_| send_one(sock_addr, contents)),
    ));
}

async fn receive_one(mut tcp_stream: TcpStream) -> io::Result<()> {
    let mut in_buf = [0; CONTENTS.len()];
    tcp_stream.read(&mut in_buf).await?;
    tcp_stream.write(&in_buf).await?;
    Ok(())
}

fn receive(sock_addr: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let receiver = TcpListener::bind(sock_addr).await.unwrap();
        let mut workers = vec![];
        for _ in 0..NUM_CONNECTIONS {
            let (s, _) = receiver.accept().await.unwrap();
            workers.push(receive_one(s));
        }
        join_all(workers).await
    });
}

fn main() {
    let start = Instant::now();

    let t1 = thread::spawn(|| send(&SOCK_ADDR, &CONTENTS));
    let t2 = thread::spawn(|| receive(&SOCK_ADDR));
    t1.join().unwrap();
    t2.join().unwrap();

    let duration = start.elapsed();
    println!("{:?}", duration);
}
