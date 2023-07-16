use std::str::from_utf8;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let receiver = TcpListener::bind("127.0.0.1:8080").await?;

    let mut sender = TcpStream::connect("127.0.0.1:8080").await?;
    let sender_res = sender.write(b"12345".as_slice());

    let (mut stream, _addr) = receiver.accept().await?;
    dbg!();
    let mut res = String::new();
    dbg!();
    sender_res.await?;
    dbg!(_addr);
    let mut buf: [u8; 5] = [0; 5];
    // DOESN'T work:
    // stream.read_to_string(&mut res).await?;
    stream.read(&mut buf).await?;
    dbg!(from_utf8(&buf).unwrap(), res);
    Ok(())
}
