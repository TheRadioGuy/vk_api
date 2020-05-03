#[tokio::main]
async fn main() {
    let mut vk_api = vkapi::VK::new("5.103", "ru");
    vk_api.set_access_token("db619ee5e3e018f25fc3a1d27153198c22d8c84850be604e3a2bb2d66a8a5d13771b56198d08ea3ac358e".to_string());

    vk_api.start_longpoll(
        194950468,
        25,
        Box::new(|event| {
            println!("callback, event: {} ", event);
        }),
    ).await;
}
