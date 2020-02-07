use crate::Message;
use futures::io::{AsyncWrite, AsyncWriteExt};
use std::io::Error;

/// A writer for SMC messages.
///
/// Consumes an [`futures::io::AsyncWrite`] to which messages will be written.
pub struct Writer<W> {
    writer: W,
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    /// Create a new message writer.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Send a message.
    ///
    /// This encodes the message, writes it and flushes the writer.
    pub async fn send(&mut self, message: Message) -> Result<(), Error> {
        send(&mut self.writer, message).await
    }

    /// Send a batch of messages.
    ///
    /// This works like [`Writer::send`] but flushes after all messages are written.
    pub async fn send_batch(&mut self, messages: Vec<Message>) -> Result<(), Error> {
        send_batch(&mut self.writer, messages).await
    }
}

pub async fn send<W>(writer: &mut W, message: Message) -> Result<(), Error>
where
    W: AsyncWrite + Unpin,
{
    let buf = message.encode()?;
    writer.write_all(&buf).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn send_batch<W>(writer: &mut W, messages: Vec<Message>) -> Result<(), Error>
where
    W: AsyncWrite + Unpin,
{
    for message in &messages {
        let buf = message.encode()?;
        writer.write_all(&buf).await?;
    }
    writer.flush().await?;
    Ok(())
}
