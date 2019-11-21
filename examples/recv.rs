use async_std::prelude::*;
use async_std::{io, task};
use simple_message_channels::{Message, Reader};

fn main() {
    task::block_on(async move {
        match recv().await {
            Err(e) => eprintln!("Error: {:?}", e),
            Ok(()) => eprintln!("Ok."),
        }
    });
}

async fn recv() -> Result<(), io::Error> {
    let stdin = io::stdin().lock().await;
    let mut reader = Reader::new(stdin);
    while let Some(msg) = reader.next().await {
        let msg = msg?;
        print_msg(&msg);
    }
    Ok(())
}

fn print_msg(msg: &Message) {
    let text = String::from_utf8(msg.message.clone()).unwrap();
    eprintln!("recv: ch {} typ {}: {}", msg.channel, msg.typ, text);
}
