use vkapi::{
    VK, param,
    types::keyboard::{Keyboard, Button, ButtonColor, TextButton, VKPayButton, LocationButton}
};
use rand::Rng;

#[tokio::main]
async fn main() {
    let access_token = std::env::var("ACCESS_TOKEN").expect("Failed to get ACCESS_TOKEN environment variable");
    let peer_id = std::env::var("PEER_ID").expect("Failed to get PEER_ID environment variable");

    let buttons = vec![
        vec![
            Button::new(LocationButton::new(""), None).unwrap(),
        ],
        vec![
            Button::new(VKPayButton::new("", "action=transfer-to-group&group_id=1&aid=10"), None).unwrap(),
        ],
        vec![
            Button::new(TextButton::new("Negative", ""), Some(ButtonColor::Negative)).unwrap(),
            Button::new(TextButton::new("Positive", ""), Some(ButtonColor::Positive)).unwrap(),
            Button::new(TextButton::new("Primary", ""), Some(ButtonColor::Primary)).unwrap(),
        ]
    ];
    let keyboard = Keyboard::new(buttons, true, None).expect("Failed to build a keyboard");
    println!("{}", keyboard.to_string());

    let mut rng = rand::thread_rng();
    let random_id: i64 = rng.gen_range(0, 50_000_000);
    let mut params = param! {"peer_id" => &peer_id, "message" => "Сообщение", "keyboard" => &keyboard.to_string(), "random_id" => &format!("{}", random_id)};

    let mut vk_api = vkapi::VK::new("5.130".to_string(), "en".to_string());
    vk_api.set_access_token(access_token);

    let response = vk_api
        .request::<serde_json::value::Value>("messages.send", &mut params)
        .await
        .unwrap();
    println!("{}", response);
}
