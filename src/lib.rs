//! Simple message channels
//!
//! A reader and writer for messages in the "simple message channels"(SMC) binary protocol. The
//! protocol encodes message in a simple pattern of (channel, type, message), where channel can
//! be any number, type can be any number between 0 and 15, and message can be any byte buffer.
//!
//! This is the basic wire protocol used by [hypercore](https://github.com/mafintosh/hypercore).
//!
//! This module is a port of the JavaScript module [of the same
//! name](https://github.com/mafintosh/simple-message-channels/).

mod message;
mod reader;
mod writer;

pub use message::Message;
pub use reader::Reader;
pub use writer::Writer;
