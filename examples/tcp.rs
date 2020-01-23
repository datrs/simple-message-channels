//! TCP example
//!
//! This demonstrates how to use simple-message-channels with TCP.
//!
//! Usage:
//! In one terminal run
//!
//! cargo run --example tcp -- server 127.0.0.1:8080
//!
//! and in another
//!
//! cargo run --example tcp -- client 127.0.0.1:8080
//!
//! This should send a message "hi" on channel 1, typ 1,
//! and the server should reply with "HI" on channel 1, typ 2.

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use async_std::task::{Context, Poll};
use futures::io::{AsyncRead, AsyncWrite, Result};
use simple_message_channels::{Message, Reader, Writer};
use std::env;
use std::io;
use std::pin::Pin;
use std::sync::Arc;

fn usage() {
    println!("usage: cargo run --example tcp -- [client|server] [address]");
    std::process::exit(1);
}

fn main() {
    let count = env::args().count();
    if count != 3 {
        usage();
    }
    let mode = env::args().nth(1).unwrap();
    let address = env::args().nth(2).unwrap();

    task::block_on(async move {
        let result = match mode.as_ref() {
            "server" => tcp_server(address).await,
            "client" => tcp_client(address).await,
            _ => panic!(usage()),
        };
        match result {
            Err(e) => eprintln!("Error: {:?}", e),
            Ok(()) => eprintln!("Ok."),
        }
    });
}

async fn tcp_server(address: String) -> Result<()> {
    let listener = TcpListener::bind(&address).await?;
    println!("Listening on {}", listener.local_addr()?);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let peer_addr = stream.peer_addr().unwrap();
        eprintln!("new connection from {}", peer_addr);
        task::spawn(async move {
            match handle_incoming(stream).await {
                Ok(()) => eprintln!("connection from {} closed without error", peer_addr),
                Err(e) => eprintln!("connection from {} closed with error: {}", peer_addr, e),
            }
        });
    }
    Ok(())
}

async fn tcp_client(address: String) -> Result<()> {
    let tcp_stream = TcpStream::connect(&address).await?;
    handle_outgoing(tcp_stream).await?;
    Ok(())
}

async fn handle_incoming(stream: TcpStream) -> Result<()> {
    let (mut reader, mut writer) = create_from_stream(stream);
    while let Some(msg) = reader.next().await {
        let msg = msg?;
        eprintln!("received: {}", format_msg(&msg));
        let resp = Message {
            channel: msg.channel,
            typ: 2,
            message: to_upper(&msg.message),
        };
        writer.send(resp).await?;
    }
    Ok(())
}

async fn handle_outgoing(stream: TcpStream) -> Result<()> {
    let (mut reader, mut writer) = create_from_stream(stream);

    let hello_msg = Message::new(1, 1, "hi".as_bytes().to_vec());
    writer.send(hello_msg).await?;

    while let Some(msg) = reader.next().await {
        let msg = msg?;
        eprintln!("received: {}", format_msg(&msg));
    }
    Ok(())
}

fn create_from_stream(tcp_stream: TcpStream) -> (Reader<CloneableStream>, Writer<CloneableStream>) {
    let stream = CloneableStream(Arc::new(tcp_stream));
    let reader = Reader::new(stream.clone());
    let writer = Writer::new(stream.clone());
    (reader, writer)
}

fn format_msg(msg: &Message) -> String {
    format!(
        "chan {} typ {} msg {}",
        msg.channel,
        msg.typ,
        String::from_utf8(msg.message.to_vec()).unwrap_or("<invalid utf8>".to_string())
    )
}

fn to_upper(bytes: &[u8]) -> Vec<u8> {
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    text.to_uppercase().as_bytes().to_vec()
}

#[derive(Clone)]
struct CloneableStream(Arc<TcpStream>);
impl AsyncRead for CloneableStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.0).poll_read(cx, buf)
    }
}
impl AsyncWrite for CloneableStream {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.0).poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.0).poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.0).poll_close(cx)
    }
}
