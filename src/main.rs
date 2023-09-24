use rgz::msgs::StringMsg;

fn main() {
    let msg = StringMsg {
        data: "Hello, world!".to_string(),
        ..Default::default()
    };
    println!("Hello, world!, {}", msg.data);
}


