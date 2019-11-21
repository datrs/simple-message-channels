use async_std::{io, task};
use simple_message_channels::{Message, Writer};

fn main() {
    task::block_on(async move { send().await.unwrap() });
}

async fn send() -> io::Result<()> {
    let stdout = io::stdout().lock().await;
    let mut writer = Writer::new(stdout);
    for i in 0..3 {
        let message = Message::new(i, 1, "hi".as_bytes().to_vec());
        print_msg(&message);
        writer.send(message).await?;
    };
    Ok(())
}

fn print_msg(msg: &Message) {
    let text = String::from_utf8(msg.message.clone()).unwrap();
    eprintln!("send: ch {} typ {}: {}", msg.channel, msg.typ, text);
}
