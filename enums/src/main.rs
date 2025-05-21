enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
    fn quit() -> Self {
        Message::Quit
    }
}

fn main() {
    let msg = Message::quit();
    //process_message(&msg);
    if msg.is_quit() {
        println!("Message is a quit message.");
    } else {
        println!("Message is not a quit message");
    }
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit!"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r,g , b) => println!("Color: rgb({}, {}, {})", r, g, b)
    }
}
