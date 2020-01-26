use crate::{Reader, Writer};
use async_std::net::TcpStream;
use async_std::task::{Context, Poll};
use futures::io::{AsyncRead, AsyncWrite};
use std::io::Result;
use std::pin::Pin;
use std::sync::Arc;

use crate::Endpoint;

pub fn create_from_stream(tcp_stream: TcpStream) -> Endpoint<ArcTcpStream, ArcTcpStream> {
    let stream = ArcTcpStream(Arc::new(tcp_stream));
    Endpoint {
        reader: Reader::new(stream.clone()),
        writer: Writer::new(stream.clone()),
    }
}

// pub struct TcpEndpoint(Endpoint<ArcTcpStream, ArcTcpStream>);
// impl TcpEndpoint {
//     pub fn from_stream(tcp_stream: TcpStream) -> Self {
//         let stream = ArcTcpStream(Arc::new(tcp_stream));
//         let endpoint = Endpoint::new(Reader::new(stream.clone()), Writer::new(stream.clone()));
//         Self(endpoint)
//     }
// }

#[derive(Clone)]
pub struct ArcTcpStream(Arc<TcpStream>);
impl AsyncRead for ArcTcpStream {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize>> {
        Pin::new(&mut &*self.0).poll_read(cx, buf)
    }
}
impl AsyncWrite for ArcTcpStream {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<Result<usize>> {
        Pin::new(&mut &*self.0).poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        Pin::new(&mut &*self.0).poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        Pin::new(&mut &*self.0).poll_close(cx)
    }
}
