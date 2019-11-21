use async_std::future::Future;
use async_std::io::{BufReader, Error};
use async_std::prelude::*;
use async_std::stream::Stream;
use async_std::task::{Context, Poll};
use futures::io::AsyncRead;
use futures::pin_mut;
use std::pin::Pin;

use crate::Message;

/// Reader for simple message channels.
///
/// Takes any [`futures::io::AsyncRead`] and is a
/// [`async_std::stream::Stream`] of [`Message`]s.
///
/// # Example
///
/// ```rust
/// use simple_message_channels::Reader;
/// let stdin = io::stdin().lock().await;
/// let mut reader = Reader::new(stdin);
/// while let Some(msg) = reader.next().await {
///     let msg = msg?;
///     println!("Received: ch {} typ {} msg {:?}", msg.channel, msg.typ, text);
/// }
/// ```
pub struct Reader<R> {
    inner: BufReader<R>,
}

impl<R> Reader<R>
where
    R: AsyncRead,
{
    /// Create a new message reader from any [`async_std::io::Read`].
    pub fn new(reader: R) -> Self {
        Self {
            inner: BufReader::new(reader),
        }
    }
}

// Proxy to the internal BufReader and decode messages.
impl<R> Stream for Reader<R>
where
    R: AsyncRead + Send + Unpin,
{
    type Item = Result<Message, Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Message, Error>>> {
        // TODO: I am very unsure if this is correct.
        // What happens here is I call the decode function, which
        // takes the reader and returns a future. The future is complete
        // when a full message has been read from the reader.
        // It seems all to work fine, however I am not sure about
        // the intrinsics: What happens if poll_next is called again?
        // I tried moving the future onto the Reader struct and replacing
        // it after being Ready, but couldn't get it to work with all the Pin
        // foobar.
        let fut = decode(&mut self.inner);
        pin_mut!(fut);
        match fut.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => Poll::Ready(Some(result)),
        }
    }
}

/// Decode a single message from a reader.
pub async fn decode<'a, R>(reader: &mut R) -> Result<Message, Error>
where
    R: AsyncRead + Unpin + 'a,
{
    // Read initial varint (message length).
    let mut varint: u64 = 0;
    let mut factor = 1;
    let mut headerbuf = vec![0u8; 1];
    loop {
        reader.read_exact(&mut headerbuf).await?;
        let byte = headerbuf[0];
        varint = varint + (byte as u64 & 127) * factor;
        if byte < 128 {
            break;
        }
        factor = factor * 128;
    }

    // Read main message.
    let mut messagebuf = vec![0u8; varint as usize];
    reader.read_exact(&mut messagebuf).await?;
    let message = Message::from_buf(&messagebuf)?;
    Ok(message)
}
