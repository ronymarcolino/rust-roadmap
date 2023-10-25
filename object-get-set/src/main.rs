#[derive(Clone, Debug)]
struct Message {
    id: i32,
    content:String,
}

impl Message {
    fn get_content(&self) -> &str {
        &self.content
    }

    fn set_content(&mut self, new_content:&str) {
        self.content = String::from(new_content)
    }
}

fn main() {
    let mut _message_1 = Message{
        id:1000,
        content: String::from("test"),
    };
    _message_1.set_content("test_2");
    println!("Message: {}", _message_1.get_content());
}
