use futures::future::join_all;
use std::thread;
use std::time::Instant;
use tokio::io;
use tokio::net::UdpSocket;

const SOCK_ADDR: &str = "127.0.0.1:9099";
const FILES: usize = 1000;
const CONTENT: [u8; 10] = [b'q'; 10];

async fn send_one(sock_addr: &str) -> io::Result<()> {
    let s = UdpSocket::bind("0.0.0.0:0").await?;
    let mut in_buf = [0; CONTENT.len()];
    s.send_to(&CONTENT, sock_addr).await?;
    let _cnt = s.recv(&mut in_buf).await?;
    assert!(in_buf.eq(&CONTENT));
    Ok(())
}

fn send(sock_addr: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(join_all((0..FILES).map(|_| send_one(sock_addr))));
}

fn receive(sock_addr: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let receiver = UdpSocket::bind(sock_addr).await.unwrap();
        for _ in 0..FILES {
            let mut in_buf = [0; CONTENT.len()];
            let (_s, addr) = receiver.recv_from(&mut in_buf).await.unwrap();
            receiver.send_to(&in_buf, addr).await.unwrap();
        }
    });
}

fn main() {
    let start = Instant::now();

    let t1 = thread::spawn(|| send(&SOCK_ADDR));
    let t2 = thread::spawn(|| receive(&SOCK_ADDR));
    t1.join().unwrap();
    t2.join().unwrap();

    let duration = start.elapsed();
    println!("{:?}", duration);
}
