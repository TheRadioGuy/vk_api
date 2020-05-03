use super::params::Params;
use super::vk::VK;
use std::sync::RwLock;
use std::thread;

pub struct Longpoll {
    server: Option<String>,
    key: Option<String>,
    group_id: String,
    wait: u16,
    token: Option<String>,
    api_version: String,
    lang: String,
}

impl Longpoll {
    pub fn new(
        group_id: u32,
        wait: u16,
        token: Option<String>,
        api_version: String,
        lang: String,
    ) -> Self {
        Self {
            server: None,
            key: None,
            group_id: group_id.to_string(),
            wait: wait,
            token,
            api_version,
            lang,
        }
    }

    pub async fn start(&self, callback: Box<dyn Fn(&json::JsonValue) -> ()>) {
        let mut params = Params::new();
        params.add_param("group_id", &self.group_id);
        let data = VK::request_public(
            "groups.getLongPollServer",
            &mut params,
            &self.token,
            &self.api_version,
            &self.lang,
        )
        .await
        .unwrap();
        let mut key = data["response"]["key"].as_str().unwrap().to_owned();
        let mut server = data["response"]["server"].as_str().unwrap().to_owned();
        let mut ts = data["response"]["ts"].as_str().unwrap().to_owned();

        loop {
            let data = Longpoll::poll(&server, &key, ts, self.wait).await;
            ts = data["ts"].as_str().unwrap().to_owned();
            let updates = &data["updates"];
            if !data["failed"].is_null() {
                let new_data = VK::request_public(
                    "groups.getLongPollServer",
                    &mut params,
                    &self.token,
                    &self.api_version,
                    &self.lang,
                )
                .await
                .unwrap();
                key = new_data["response"]["key"].as_str().unwrap().to_owned();
                server = new_data["response"]["server"].as_str().unwrap().to_owned();
                ts = new_data["response"]["ts"].as_str().unwrap().to_owned();
                continue;
            }

            updates.members().for_each(|event| {
                callback(event);
            });
        }
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
