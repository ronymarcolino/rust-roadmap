#[derive(Clone, Debug)]
struct Message{
    id: i32,
    content:String,
}

fn main() {
    let message_1 = Message{
        id:1000,
        content: String::from("test"),
    };
    let message_2 = Message{
        id:1001,
        content: String::from("teste_1"),
    };
    let mut message_3 = Box::new(message_1.clone());
    message_3.id = 1002;
    message_3.content = String::from("new message");
    println!("Message 1: {}, {}", message_1.id, message_1.content);
    println!("Message 2: {}, {}", message_2.id, message_2.content);
    println!("Message 3: {}, {}", message_3.id, message_3.content);
}
