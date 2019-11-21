use async_std::prelude::*;
use async_std::{io, task};
use simple_message_channels::{Message, Reader, Writer};

fn main() {
    task::block_on(async move {
        match echo().await {
            Err(e) => eprintln!("Error: {:?}", e),
            Ok(()) => eprintln!("Ok."),
        }
    });
}

async fn echo() -> Result<(), io::Error> {
    let stdin = io::stdin().lock().await;
    let stdout = io::stdout().lock().await;
    let mut reader = Reader::new(stdin);
    let mut writer = Writer::new(stdout);
    while let Some(msg) = reader.next().await {
        let msg = msg?;
        let resp = Message {
            channel: msg.channel,
            typ: msg.typ + 1,
            message: to_upper(&msg.message),
        };
        writer.send(resp).await?;
    }
    Ok(())
}

fn to_upper(bytes: &[u8]) -> Vec<u8> {
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    text.to_uppercase().as_bytes().to_vec()
}
