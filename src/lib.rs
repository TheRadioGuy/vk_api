#[macro_use]
extern crate json;
use std::collections::HashMap;

type Request = String;

pub struct VK {
    access_token: Option<String>,
    api_version: String,
    language: String,
}

impl VK {
    pub fn new(api_version: &str, language: &str) -> Self {
        Self{access_token: None, api_version: api_version.to_owned(), language: language.to_owned()}
    }

    pub fn set_access_token(&mut self, token: &str){
        self.access_token = Some(token.to_string())
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

    pub async fn direct_auth(&mut self, login: &str, password: &str) -> Result<(),  json::JsonValue> {
        let url = format!("https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username={}&password={}", login, password);
        let response = reqwest::get(&url)
        .await.unwrap().text().await.unwrap();
        let parsed = json::parse(&response).unwrap();
        if parsed["access_token"].is_null() {
            panic!("Authencication has failed! We doesnt support 2fa right now");
            return Err(parsed);
        }

        let mut access_token = parsed["access_token"].as_str().unwrap();
        self.set_access_token(access_token);

        Ok(())
    }

    fn build_request(&self, method: &str, params: HashMap<&str, &str>) -> Request {
        let access_token = self.access_token.as_ref();
        if access_token.is_none(){
            panic!("Access token is empty! Did you forget to call set_access_token() ?");
        } 

        let access_token = access_token.unwrap();
        let mut result = Request::from("https://api.vk.com/method/");
        result.push_str(method);
        result.push_str("?");
        params.iter().for_each(|(p, v)| result.push_str(&format!("{}={}&", p, v)) ); // Make parametres to one string
        result.push_str(&format!("access_token={}", &access_token));
        result.push_str("&v=");
        result.push_str(&self.api_version);
        result
    }
}


