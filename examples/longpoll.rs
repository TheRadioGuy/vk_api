use vkapi::longpoll::EventType;

#[tokio::main]
async fn main() {
    let mut vk_api = vkapi::VK::new("5.103", "ru", "db619ee5e3e018f25fc3a1d27153198c22d8c84850be604e3a2bb2d66a8a5d13771b56198d08ea3ac358e".into());
    let ch = vk_api.start_longpoll(
        194950468,
        25,
    );

    for event in ch {
        match event.0 {
            EventType::NewMessage => println!("new message: {}", event.1),
            EventType::Other(name) => println!("new event type {} {}", name, event.1)
        };
    }
}
