// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message<'a> {
     Move{x: u32, y: u32},
     Echo(&'a str),
     ChangeColor(u8,u8,u8),
     Quit,           
}

impl Message<'_> {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let var_name = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(&String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];
    let messages = var_name;

    for message in &messages {
        message.call();
    }
}
