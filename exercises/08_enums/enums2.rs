#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: 定义下面所使用的不同变体(variants)。
    // Quit 变体不包含任何数据。
    Quit,
    // Move 变体包含一个 Point 结构体。
    Move(Point),
    // Echo 变体包含一个字符串。
    Echo(String),
    // ChangeColor 变体包含三个 u8 值。
    ChangeColor(u8, u8, u8),
    // Resize 变体包含两个 u64 值。
    Resize { width: u64, height: u64 },
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
