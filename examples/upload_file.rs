use vkapi::types::{destination::Destination, file::File};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut vk_api = vkapi::VK::new("5.103", "ru");
    vk_api.set_access_token(
        "db619ee5e3e018f25fc3a1d27153198c22d8c84850be604e3a2bb2d66a8a5d13771b56198d08ea3ac358e"
            .into(),
    );
    let file = File::new("examples/1.jpg", Destination::Message);

    let r = vk_api.upload(file).await.unwrap();
    println!("{}", r);
}
