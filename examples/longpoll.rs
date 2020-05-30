use vkapi::LongpollEvent;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let mut vk_api = vkapi::VK::new("5.103".to_owned(), "ru".to_owned());
    vk_api.set_access_token("db619ee5e3e018f25fc3a1d27153198c22d8c84850be604e3a2bb2d66a8a5d13771b56198d08ea3ac358e".to_string());

    let mut stream = vk_api.init_stream(194950468, 25).await;

    stream.set_prefix("<префикс бота>");
    stream.set_allowed_events(&[LongpollEvent::MessageNew]);

    let mut stream = stream.build_stream();

    while let Some(update) = stream.next().await {
        // handle longpoll update
    }
}
