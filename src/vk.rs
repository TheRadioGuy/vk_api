use futures::future::Future;
use crate::params::Params;
use crate::longpoll::Longpoll;

/// Request type - used for make request to VK API
type Request = String;

/// VK structure, used for call api method, use longpoll api and etc
pub struct VK<'a> {
    access_token: Option<String>, // todo: make it's &str
    api_version: &'a str,
    language: &'a str,
}

impl<'a> VK<'a> {
    /// Create new instance of `VK` struct
    /// 
    /// # Arguments: 
    /// * `api_version` - pick up one [here](https://vk.com/dev/versions)
    /// * `language` - [here](https://vk.com/dev/api_requests)
    pub fn new(api_version: &'a str, language: &'a str) -> Self {
        Self {
            access_token: None,
            api_version,
            language
        }
    }

    /// Set access token
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }

    /// Get a reference to token
    pub fn get_access_token(&self) -> &String {
        self.access_token.as_ref().unwrap()
    }

    ///  This methond starts to longpolling, It's sad, but as for now there're ways to stop it (but I'm gonna make it)
    /// # Arguments:
    /// * `group_id` - your group ID
    /// * `wait` - Maximal time to waiting, max value is 90
    /// * `callback` - closure which have 1 argument: [event](https://vk.com/dev/groups_events)
    pub fn start_longpoll(&self, group_id: u32, wait: u16, callback: Box<dyn Fn(&json::JsonValue) -> ()>) -> Longpoll {
        let access_token = self.access_token.clone();
        let longpoll = Longpoll::new(group_id, wait, access_token, self.api_version.to_string().clone(), self.language.to_string().clone());
        longpoll.start(callback);
        longpoll
    }

    /// Used for request API
    /// # Arguments:
    /// * `method` - all methods are listed [here](https://vk.com/dev/methods)
    /// * `params` - params to call API. use `Params::new()` and then `params.add()`
    #[tokio::main]
    pub async fn request(
        &self,
        method: &str,
        params: &mut Params,
    ) -> std::result::Result<json::JsonValue, String> {
        let request_url = self.build_request(method, params);
        let response = reqwest::get(&request_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let parsed = json::parse(&response).unwrap(); // TODO: Get rid of unwrap (need help)
                                                      // Check if it's error
        if !parsed["error"].is_null() {
            return Err(parsed["error"]["error_msg"].as_str().unwrap().to_owned());
        }

        Ok(parsed)
    }

    /// ** I'm not recommend use it **
    /// You can use `request` but without constructing VK instance
    #[tokio::main]
    pub async fn request_public( // TODO: get rid of shitcode
        method: &str,
        params: &mut Params,
        access_token: &Option<String>,
        api_version: &str,
        language: &str
    ) -> std::result::Result<json::JsonValue, String> {
        let request_url = {
            if access_token.is_none() {
                panic!("Access token is empty! Did you forget to call set_access_token() ?");
            }
            let access_token = access_token.as_ref().unwrap();
            let result = format!("https://api.vk.com/method/{}?{}access_token={}&v={}&lang={}", method, &params.concat(), &access_token, api_version, language);
            result
        };

        let response = reqwest::get(&request_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let parsed = json::parse(&response).unwrap(); // TODO: Get rid of unwrap (need help)
        if !parsed["error"].is_null() {
            return Err(parsed["error"]["error_msg"].as_str().unwrap().to_owned());
        }
        Ok(parsed)
    }

    /// Used for direct auth
    /// * `login` - email or phone number
    /// * `password` - Hm..what it's supposed to be
    #[tokio::main]
    pub async fn direct_auth(
        &mut self,
        login: &str,
        password: &str,
    ) -> Result<(), json::JsonValue> {
        let url = format!("https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username={}&password={}", login, password);
        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let parsed = json::parse(&response).unwrap();
        if parsed["access_token"].is_null() {
            panic!("Authencication has failed! We doesnt support 2fa right now");
        }

        let access_token = parsed["access_token"].as_str().unwrap();
        let access_token = access_token.to_string();
        self.set_access_token(access_token);

        Ok(())
    }

    fn build_request(&self, method: &str, params: &mut Params) -> Request {
        let access_token = self.access_token.as_ref();
        if access_token.is_none() {
            panic!("Access token is empty! Did you forget to call set_access_token() ?");
        }

        let access_token = access_token.unwrap();
        let result = format!("https://api.vk.com/method/{}?{}access_token={}&v={}&lang={}", method, &params.concat(), &access_token, &self.api_version, &self.language);
        result
    }
}
