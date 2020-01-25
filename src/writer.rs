use futures::io::{AsyncWrite, AsyncWriteExt, BufWriter};
use std::io::Error;
use std::sync::{Arc, RwLock};

use crate::cipher::{Cipher, SharedCipher};
use crate::Message;

/// A writer for SMC messages.
///
/// Consumes an [`futures::io::AsyncWrite`] to which messages will be written.
pub struct Writer<W> {
    writer: BufWriter<W>,
    cipher: SharedCipher,
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    /// Create a new message writer.
    pub fn new(writer: W) -> Self {
        Self::encrypted(writer, Arc::new(RwLock::new(Cipher::empty())))
    }

    pub fn encrypted(writer: W, cipher: SharedCipher) -> Self {
        Self {
            writer: BufWriter::new(writer),
            cipher,
        }
    }

    /// Send a message.
    ///
    /// This encodes the message, writes it and flushes the writer.
    pub async fn send(&mut self, message: Message) -> Result<(), Error> {
        let mut buf = message.encode()?;
        self.cipher
            .write()
            .expect("could not aquire lock")
            .try_apply(&mut buf);
        self.writer.write_all(&buf).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Send a batch of messages.
    ///
    /// This works like [`Writer::send`] but flushes after all messages are written.
    pub async fn send_batch(&mut self, messages: Vec<Message>) -> Result<(), Error> {
        for message in &messages {
            let mut buf = message.encode()?;
            self.cipher
                .write()
                .expect("could not aquire lock")
                .try_apply(&mut buf);
            self.writer.write_all(&buf).await?;
        }
        self.writer.flush().await?;
        Ok(())
    }
}
