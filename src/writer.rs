use crate::Message;
use async_std::io::{BufWriter, Error};
use async_std::prelude::*;
use futures::io::AsyncWrite;

/// A writer for SMC messages.
///
/// Consumes an [`futures::io::AsyncWrite`] to which messages will be written.
pub struct Writer<W> {
    writer: BufWriter<W>,
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    /// Create a new message writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    /// Send a message.
    ///
    /// This encodes the message, writes it and flushes the writer.
    pub async fn send(&mut self, message: Message) -> Result<(), Error> {
        let buf = message.encode()?;
        self.writer.write_all(&buf).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Send a batch of messages.
    ///
    /// This works like [`Writer::send`] but flushes after all messages are written.
    pub async fn send_batch(&mut self, messages: Vec<Message>) -> Result<(), Error> {
        for message in &messages {
            let buf = message.encode()?;
            self.writer.write_all(&buf).await?;
        }
        self.writer.flush().await?;
        Ok(())
    }
}
