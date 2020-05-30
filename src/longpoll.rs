use super::params::Params;
use super::vk::VK;
use crate::types::longpoll::{
    FailedLongpollResponse, LongpollResponse, LongpollServerResponse, LongpollUpdate,
    SuccessfulLongpollResponse,
};
use crate::utils::LongpollStream;
use anyhow::Context as _;
use futures::channel::mpsc::Sender;
use futures::{SinkExt};

pub struct Longpoll {
    group_id: String,
    wait: u8,
    token: Option<String>,
    api_version: String,
    lang: String,
}

impl Longpoll {
    pub fn new(
        group_id: u32,
        wait: u8,
        token: Option<String>,
        api_version: String,
        lang: String,
    ) -> Self {
        Self {
            group_id: group_id.to_string(),
            wait,
            token,
            api_version,
            lang,
        }
    }

    // TODO: currently we can't properly handle errors in the spawned tokio task, need to resolve
    pub async fn start(self) -> LongpollStream {
        let mut params = Params::new();
        params.add_param("group_id", &self.group_id);
        log::trace!("{}:{} Longpoll.start", file!(), line!());
        let data: LongpollServerResponse = VK::request_public(
            "groups.getLongPollServer",
            &params,
            &self.token,
            &self.api_version,
            &self.lang,
        )
        .await
        .unwrap();

        let (s, r) = futures::channel::mpsc::channel(10);

        // error handler
        tokio::spawn(async move {
            let key = data.response.key;
            let server = data.response.server;
            let ts = data.response.ts;

            let token = self.token;
            let params = params;
            let lang = self.lang;
            let api_version = self.api_version;

            loop {
                match poller(
                    server.clone(),
                    key.clone(),
                    ts.clone(),
                    token.clone(),
                    params.clone(),
                    lang.clone(),
                    api_version.clone(),
                    self.wait,
                    s.clone(),
                )
                    .await {
                    Err(e) => {
                        log::error!("Error in longpoll loop! {:?}", e);
                        continue;
                    }
                    _ => continue
                }
            }
        });

        LongpollStream::new(r)
    }

    pub fn stop() {
        unimplemented!();
    }

    async fn poll(
        server: &str,
        key: &str,
        ts: &str,
        wait: u8,
    ) -> Result<LongpollResponse, anyhow::Error> {
        let url = format!("{}?act=a_check&key={}&ts={}&wait={}", server, key, ts, wait);
        let response: String = reqwest::get(&url)
            .await
            .context(format!(
                "{}:{} Longpoll::poll | could not complete get request",
                file!(),
                line!()
            ))?
            .text()
            .await
            .context(format!(
                "{}:{} Longpoll::poll | could not parse response into string",
                file!(),
                line!()
            ))?;
        log::trace!(
            "{}:{} Longpoll::poll | response: {}",
            file!(),
            line!(),
            &response
        );
        let parsed = serde_json::from_str::<SuccessfulLongpollResponse>(&response);

        match parsed {
            Ok(success) => Ok(LongpollResponse::Success(success)),
            Err(e) => {
                let res = serde_json::from_str::<FailedLongpollResponse>(&response);

                match res {
                    Ok(err) => Ok(LongpollResponse::Fail(err)),
                    Err(_) => Err(e.into()),
                }
            }
        }
    }
}

async fn poller(
    server: String,
    key: String,
    ts: String,
    token: Option<String>,
    params: Params,
    lang: String,
    api_version: String,
    wait: u8,
    mut s: Sender<LongpollUpdate>,
) -> Result<(), anyhow::Error> {
    let mut server = server;
    let mut key = key;
    let mut ts = ts;

    loop {
        log::trace!("longpoll loop");
        use LongpollResponse::*;
        let data = Longpoll::poll(&server, &key, &ts, wait).await?;

        let mut updates = Vec::new();

        match data {
            Success(data) => {
                log::trace!("{}:{} {:?} Poll::loop.data", file!(), line!(), &data);
                ts = data.ts;
                updates = data.updates;
            }
            Fail(_) => {
                let new_data: LongpollServerResponse = VK::request_public(
                    "groups.getLongPollServer",
                    &params,
                    &token,
                    &api_version,
                    &lang,
                )
                .await?;
                key = new_data.response.key;
                server = new_data.response.server;
                ts = new_data.response.ts;
                continue;
            }
        }

        for event in updates {
            s.send(event.clone()).await?;
        }
    }
}
