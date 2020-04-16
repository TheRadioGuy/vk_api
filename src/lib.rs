#[macro_use]
extern crate json;
use std::collections::HashMap;

type Request = String;

pub struct VK {
    access_token: String,
    api_version: String,
    language: String,
}

impl VK {
    pub fn new(access_token: &str, api_version: &str, language: &str) -> Self {
        Self{access_token: access_token.to_owned(), api_version: api_version.to_owned(), language: language.to_owned()}
    }

    #[tokio::main]
    pub async fn request(&self, method: &str, params: HashMap<&str, &str>) -> std::result::Result<json::JsonValue, json::JsonValue>{
        let request_url = self.build_request(method, params);
        let response = reqwest::get(&request_url)
        .await.unwrap().text().await.unwrap();
        let parsed = json::parse(&response).unwrap(); // TODO: Get rid of unwrap (need help tho)
        // Check if it's error
        if !parsed["error"].is_null(){
            return Err(parsed);
        }

        Ok(parsed)
    }

    fn build_request(&self, method: &str, params: HashMap<&str, &str>) -> Request {
        let mut result = Request::from("https://api.vk.com/method/");
        result.push_str(method);
        result.push_str("?");
        params.iter().for_each(|(p, v)| result.push_str(&format!("{}={}&", p, v)) ); // Make parametres to one string
        result.push_str(&format!("access_token={}", self.access_token));
        result.push_str("&v=");
        result.push_str(&self.api_version);
        result
    }
}


