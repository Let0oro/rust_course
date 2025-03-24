#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment (&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(self) -> String {
        self.time.clone()
    }
}

pub fn main () {
    println!("Exercise:\n");

    let str_slice = String::from("hola bb");

    let chat_with_sergio = ChatMessage {
        content: &str_slice[4..], time: String::from("04:00")
    };

    let chat_with_david = ChatMessage {
        content: "qué tal python?", time: String::from("12:00")
    };

    let chat_with_isabel = ChatMessage {
        content: DigitalContent::AudioFile, time: String::from("10:00")
    };

    chat_with_isabel.consume_entertainment();

    println!("Talking with Sergio at {:?} hours", chat_with_sergio.retrieve_time());
    println!("Talking with David at {:?} hours", chat_with_david.retrieve_time());
    println!("Talking with Isabel at {:?} hours", chat_with_isabel.retrieve_time());

    println!("---\n")
}