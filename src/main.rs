#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl<T: std::fmt::Debug> ChatMessage<T> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(self) -> String {
        self.time.clone()
    } 
}

fn main() {
    let example: ChatMessage<DigitalContent> = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("9:00PM"),
    };
    let alpha: ChatMessage<&str> = ChatMessage {
        content: "Howdy",
        time: String::from("8:45PM")
    };
    let beta: ChatMessage<String> = ChatMessage {
        content: String::from("What you doin tonight?"),
        time: String::from("9:03PM"),
    };

    example.consume_entertainment();
    println!("{:?}", example.retrieve_time());
    println!("{:?}", alpha.retrieve_time());
    println!("{:?}", beta.retrieve_time());
}