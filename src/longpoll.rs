use super::params::Params;
use super::vk::VK;
use std::sync::RwLock;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

/// TODO: Add more event types
#[derive(Debug, Clone)]
pub enum EventType {
    NewMessage,
    Other(String)
}

impl EventType {
    pub fn new(event: &str) -> Self {
        match event {
            "message_new" => EventType::NewMessage,
            _ => EventType::Other(event.to_owned())
        }
    }
}
pub struct Longpoll {
    group_id: String,
    wait: u16,
    token: String,
    api_version: String,
    lang: String,
}

impl Longpoll {
    pub fn new(
        group_id: u32,
        wait: u16,
        token: String,
        api_version: String,
        lang: String,
    ) -> Self {
        Self {
            group_id: group_id.to_string(),
            wait: wait,
            token,
            api_version,
            lang,
        }
    }

    pub fn start(&self) -> Receiver<(EventType, json::JsonValue)> {
        let (tx, rx) = channel();
        let token = self.token.clone();
        let api_version = self.api_version.clone();
        let lang = self.lang.clone();
        let group_id = self.group_id.clone();
        let wait = self.wait;

        tokio::spawn(async move {
        let mut params = Params::new();
        params.add_param("group_id", &group_id);
        let data = VK::request_public(
            "groups.getLongPollServer",
            &mut params,
            &token,
            &api_version,
            &lang,
        )
        .await
        .unwrap();
        let mut key = data["response"]["key"].as_str().unwrap().to_owned();
        let mut server = data["response"]["server"].as_str().unwrap().to_owned();
        let mut ts = data["response"]["ts"].as_str().unwrap().to_owned();

        loop {
            let data = Longpoll::poll(&server, &key, ts, wait).await;
            ts = data["ts"].as_str().unwrap().to_owned();
            let updates = &data["updates"];
            if !data["failed"].is_null() {
                let new_data = VK::request_public(
                    "groups.getLongPollServer",
                    &mut params,
                    &token,
                    &api_version,
                    &lang,
                )
                .await
                .unwrap();
                key = new_data["response"]["key"].as_str().unwrap().to_owned();
                server = new_data["response"]["server"].as_str().unwrap().to_owned();
                ts = new_data["response"]["ts"].as_str().unwrap().to_owned();
                continue;
            }

            updates.members().for_each(|event| {
                let event_type = EventType::new(&event["type"].as_str().unwrap());
                let event = event["object"].clone();
                tx.send((event_type, event)).unwrap();
            });
        }
        });

        rx
    }

    pub fn stop() {
        unimplemented!();
    }


    pub async fn poll(server: &String, key: &String, ts: String, wait: u16) -> json::JsonValue {
        let url = format!("{}?act=a_check&key={}&ts={}&wait={}", server, key, ts, wait);
        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let parsed = json::parse(&response).unwrap();
        parsed
    }
}
