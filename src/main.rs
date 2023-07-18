use futures::future::join_all;
use std::thread;
use std::time::Instant;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const SOCK_ADDR: &str = "127.0.0.1:9009";
const NUM_CONNECTIONS: usize = 1000;
const CONTENTS: [u8; 10] = [b'q'; 10];

async fn send_one(sock_addr: &str, contents: &[u8]) -> io::Result<()> {
    let mut s = TcpStream::connect(sock_addr).await?;
    let mut in_buf = [0; CONTENTS.len()];
    s.write(contents).await?;
    let _cnt = s.read(&mut in_buf).await?;
    Ok(())
}

fn send(rt: &tokio::runtime::Runtime) {
    rt.block_on(join_all(
        (0..NUM_CONNECTIONS).map(|_| async{tokio::spawn(send_one(SOCK_ADDR, &CONTENTS))}),
    ));
}

async fn receive_one(mut tcp_stream: TcpStream) -> io::Result<()> {
    let mut in_buf = [0; CONTENTS.len()];
    tcp_stream.read(&mut in_buf).await?;
    tcp_stream.write(&in_buf).await?;
    Ok(())
}

fn receive(rt: &tokio::runtime::Runtime) {
    rt.block_on(async {
        let receiver = TcpListener::bind(SOCK_ADDR).await.unwrap();
        let mut join_handles = vec![];
        for _ in 0..NUM_CONNECTIONS {
            let (s, _) = receiver.accept().await.unwrap();
            join_handles.push(async{tokio::spawn(receive_one(s))});
        }
        join_all(join_handles).await
    });
}

fn main() {
    let start = Instant::now();

    let rt = tokio::runtime::Runtime::new().unwrap();

    thread::scope(|s| {
        s.spawn(|| send(&rt));
        s.spawn(|| receive(&rt));
    });

    let duration = start.elapsed();
    println!("{:?}", duration);
}
