// Exercise to learn enum, used with pattern matching

pub enum Message {
    Ping,
    Pong,
    Text(String),
    Close
}

pub fn manage_message(msg: Message) {
    match msg {
        Message::Ping => println!("Ping!"),
        Message::Pong => println!("Pong!"),
        Message::Text(m) => println!("Message text: << {m} >>"),
        _ => println!("Close!")
    }
}