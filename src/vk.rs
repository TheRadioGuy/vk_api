use reqwest::multipart::Form; // FIXME: Add non-blocking

use crate::longpoll::Longpoll;
use crate::params::Params;
use crate::types::destination::Destination;
use crate::types::{
    PhotoSaveResponse, PhotoUploadResponse, PhotoUploadServerResponse, VkError,
};
use crate::utils::{IntoPart, LongpollStream};
use anyhow::Context;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// Request type - used for make request to VK API
type Request = String;

/// VK structure, used for call api method, use longpoll api and etc
pub struct VK {
    access_token: Option<String>,
    api_version: String,
    language: String,
}

impl VK {
    /// Create new instance of `VK` struct
    ///
    /// # Arguments:
    /// * `api_version` - pick up one [here](https://vk.com/dev/versions)
    /// * `language` - [here](https://vk.com/dev/api_requests)
    pub fn new(api_version: String, language: String) -> Self {
        Self {
            access_token: None,
            api_version,
            language,
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
    pub async fn init_stream(&self, group_id: u32, wait: u8) -> LongpollStream {
        let access_token = self.access_token.clone();
        let longpoll = Longpoll::new(
            group_id,
            wait,
            access_token,
            self.api_version.to_string(),
            self.language.to_string(),
        );
        longpoll.start().await
    }

    /// Use if you want to upload any file
    /// # Arguments:
    /// [`file`] - is File struct
    ///
    /// # Return:
    /// this method returns JsonValue like this:
    ///  ``` [{"id":457239436,"album_id":-64,"owner_id":-142102660,"user_id":100,"sizes":[{"type":"s","url":"https://sun9-64.userapi.com/c857724/v857724964/1cd30a/OkprPbgIA4M.jpg","width":75,"height":42},{"type":"m","url":"https://sun9-11.userapi.com/c857724/v857724964/1cd30b/zf7PALt9LiQ.jpg","width":130,"height":73},{"type":"x","url":"https://sun9-42.userapi.com/c857724/v857724964/1cd30c/QkU_GWrWrtg.jpg",":"o","url":"https://sun9-55.userapi.com/c857724/v857724964/1cd30f/VuJoCsfzNhQ.jpg","width":130,"height":87},{"type":"p","url":"https://sun9-7.userapi.com/c857724/v857724964/1cd310/03M7NFSwldw.jpg","width":200,"height":133},{"type":"q","url":"https://sun9-27.userapi.com/c857724/v857724964/1cd311/E-l93bSWpSk.jpg","width":320,"height":213},{"type":"r","url":"https://sun9-61.userapi.com/c857724/v857724964/1cd312/0aqWUY6M9jE.jpg","width":510,"height":340}],"text":"","date":1587293486,"access_key":"cb0b2221048b3d8021"}]     ```
    /// More details about return [here](https://vk.com/dev/upload_files)

    // TODO: generic
    pub async fn save_photo(
        &self,
        resp: PhotoUploadResponse,
    ) -> Result<PhotoSaveResponse, anyhow::Error> {
        let mut params = Params::new();
        params.add_param("server", &resp.server.to_string());
        params.add_param("photo", &resp.photo);
        params.add_param("hash", &resp.hash);
        let res = self
            .request(Destination::Message.pick_method_save(), &mut params)
            .await?;

        Ok(res)
    }

    // TODO: support for generic upload, right now - only photos
    pub async fn upload(
        &self,
        file: impl IntoPart,
        destination: Destination,
        // params: Params
    ) -> Result<PhotoUploadResponse, anyhow::Error> {
        let mut form = Form::new();
        let mut params = Params::new();
        let upload: PhotoUploadServerResponse = self
            .request(destination.pick_method_load(), &mut params)
            .await?;

        let upload_url = upload.response.upload_url;
        let param_for_sending = destination.pick_param();
        form = form.part(param_for_sending, file.into_part(None).await?);
        let client = reqwest::Client::new();
        let req = client.post(&upload_url).multipart(form);
        let response = req.send().await?.text().await?;

        let parsed = serde_json::from_str(&response);

        cvt(parsed, &response)

        // if method_for_saving != "none" {
        //     let mut params = Params::new();
        //     crate::utils::make_params_for_saving_photo(&mut params, upload_info);
        //     let mut saved_file = self.request(method_for_saving, &mut params).await?;
        //     return Ok(saved_file.remove("response"));
        // }
    }

    pub async fn request_post<T: DeserializeOwned + Debug>(
        &self,
        method: &str,
        params: Params,
    ) -> Result<T, anyhow::Error> {
        let url = format!("https://api.vk.com/method/{}", method);

        let mut params = params;

        params.add_param("v", &self.api_version);
        params.add_param(
            "access_token",
            &self
                .access_token
                .as_ref()
                .context("You need to set the access_token!")?,
        );

        let client = reqwest::Client::new();

        let response: String = client
            .post(&url)
            .form(params.get_params())
            .send()
            .await
            .context(format!(
                "{}:{} VK::request | could not complete get request",
                file!(),
                line!()
            ))?
            .text()
            .await
            .context(format!(
                "{}:{} VK::request | could not parse response into string",
                file!(),
                line!()
            ))?;

        log::trace!(
            "{}:{} VK::request | response: {}",
            file!(),
            line!(),
            &response
        );

        let parsed = serde_json::from_str(&response);

        cvt(parsed, &response)
    }

    /// Used for request API
    /// # Arguments:
    /// * `method` - all methods are listed [here](https://vk.com/dev/methods)
    /// * `params` - params to call API. use `Params::new()` and then `params.add()`
    pub async fn request<T: DeserializeOwned + Debug>(
        &self,
        method: &str,
        params: &mut Params,
    ) -> std::result::Result<T, anyhow::Error> {
        let request_url = self.build_request(method, params)?;
        let response: String = reqwest::get(&request_url)
            .await
            .context(format!(
                "{}:{} VK::request | could not complete get request",
                file!(),
                line!()
            ))?
            .text()
            .await
            .context(format!(
                "{}:{} VK::request | could not parse response into string",
                file!(),
                line!()
            ))?;

        log::trace!(
            "{}:{} VK::request | response: {}",
            file!(),
            line!(),
            &response
        );

        let parsed = serde_json::from_str(&response);

        cvt(parsed, &response)
    }

    /// ** I'm not recommend use it **
    /// You can use `request` but without constructing VK instance
    pub async fn request_public<T: DeserializeOwned + Debug>(
        // TODO: get rid of shitcode
        method: &str,
        params: &Params,
        access_token: &Option<String>,
        api_version: &str,
        language: &str,
    ) -> std::result::Result<T, anyhow::Error> {
        let request_url = {
            let access_token = access_token
                .as_ref()
                .context("You need to set the access_token!")?;
            let result = format!(
                "https://api.vk.com/method/{}?{}access_token={}&v={}&lang={}",
                method,
                &params.concat(),
                &access_token,
                api_version,
                language
            );
            result
        };

        let response: String = reqwest::get(&request_url)
            .await
            .context(format!(
                "{}:{} VK::request_public | could not complete get request",
                file!(),
                line!()
            ))?
            .text()
            .await
            .context(format!(
                "{}:{} VK::request_public | could not parse response into string",
                file!(),
                line!()
            ))?;

        let parsed = serde_json::from_str(&response);
        log::trace!(
            "{}:{} VK::request_public | parsed_res: {:?}",
            file!(),
            line!(),
            &parsed
        );

        cvt(parsed, &response)
    }

    /// Used for direct auth
    /// * `login` - email or phone number
    /// * `password` - Hm..what it's supposed to be

    // TODO: refactor lately (better to eliminate direct auth at all)
    // pub async fn direct_auth(
    //     &mut self,
    //     login: &str,
    //     password: &str,
    // ) -> Result<(), anyhow::Error> {
    //     let url = format!("https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username={}&password={}", login, password);
    //     let response: String = reqwest::get(&url)
    //         .await.context(format!("{}:{} VK::direct_auth | could not complete get request", file!(), line!()))?
    //         .text()
    //         .await.context(format!("{}:{} VK::direct_auth | could not parse response into string", file!(), line!()))?;
    //     let parsed = serde_json::from_str(&response)
    //     if parsed["access_token"].is_null() {
    //         panic!("Authencication has failed! We doesnt support 2fa right now");
    //     }
    //
    //     let access_token = parsed["access_token"].as_str().unwrap();
    //     let access_token = access_token.to_string();
    //     self.set_access_token(access_token);
    //
    //     Ok(())
    // }

    fn build_request(&self, method: &str, params: &mut Params) -> Result<Request, anyhow::Error> {
        let access_token = self
            .access_token
            .as_ref()
            .context("You need to set the access_token!")?;

        let result = format!(
            "https://api.vk.com/method/{}?{}access_token={}&v={}&lang={}",
            method,
            &params.concat(),
            &access_token,
            &self.api_version,
            &self.language
        );

        Ok(result)
    }
}

fn cvt<T: DeserializeOwned + Debug>(
    parsed: serde_json::Result<T>,
    response: &str,
) -> std::result::Result<T, anyhow::Error> {
    log::trace!("{}:{} cvt::parsed: {:?}", file!(), line!(), &parsed);
    match parsed {
        Err(e) => {
            if e.is_data() {
                match serde_json::from_str::<VkError>(&response) {
                    Ok(vk) => anyhow::bail!(vk.error_msg),
                    Err(e) => Err(e.into()),
                }
            } else {
                anyhow::bail!("Unknown error!")
            }
        }
        Ok(parsed) => Ok(parsed),
    }
}
