use std::str::from_utf8;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let receiver = TcpListener::bind("localhost:8080").await?;
    dbg!(receiver.local_addr().unwrap());

    let mut sender = TcpStream::connect("[::1]:8080").await?;
    let sender_res = sender.write(b"12345".as_slice());

    let (mut stream, _addr) = receiver.accept().await?;
    sender_res.await?;
    dbg!(_addr);
    let mut buf: [u8; 5] = [0; 5];
    stream.read(&mut buf).await?;
    dbg!(from_utf8(&buf).unwrap());
    Ok(())
}
