// Copy-pasted from https://github.com/teloxide/teloxide/blob/master/src/requests/utils.rs, thanks guys
use std::{borrow::Cow, path::PathBuf};

use crate::types::longpoll::LongpollUpdate;
use bytes::{Bytes, BytesMut, Buf};
use futures::channel::mpsc::Receiver;
use futures::task::{Context, Poll};
use futures::{Stream, StreamExt, AsyncReadExt, FutureExt};
use reqwest::{multipart::Part, Body};
use reqwest::Response;
use std::future::Future;
use std::pin::Pin;
use tokio_util::codec::{Decoder, FramedRead, BytesCodec};
use anyhow::Error;
use crate::{LongpollEvent};
use std::sync::{Arc, Mutex, RwLock};
use image::{RgbaImage, ImageBuffer, GenericImage, GenericImageView, DynamicImage};
use futures::future::BoxFuture;
use rand::{random, AsByteSliceMut};
use anyhow::Context as _;
use std::io::{Cursor, Read};

struct FileDecoder;

impl Decoder for FileDecoder {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }
        Ok(Some(src.split().freeze()))
    }
}

pub async fn image_from_response(response: Response) -> Result<DynamicImage, anyhow::Error>
{
    let bytes = response
        .bytes()
        .await
        .context(format!(
            "{}:{} Utils::image_from_response | could not turn response into bytes",
            file!(),
            line!()
        ))?;
    Ok(image::load_from_memory(&bytes)?)
}

// pub fn make_params_for_saving_photo(
//     params: &mut crate::params::Params,
//     upload_info: json::JsonValue,
// ) {
//     if !upload_info["server"].is_null() {
//         params.add_param(
//             "server",
//             &upload_info["server"].as_u32().unwrap().to_string(),
//         );
//     }
//     if !upload_info["aid"].is_null() {
//         params.add_param("aid", upload_info["aid"].as_str().unwrap());
//     }
//     if !upload_info["photo"].is_null() {
//         params.add_param("photo", upload_info["photo"].as_str().unwrap());
//     }
// }

pub struct LongpollStream {
    pub(crate) inner: Receiver<LongpollUpdate>,
    prefixes: Arc<RwLock<Vec<String>>>,
    events: Arc<RwLock<Vec<LongpollEvent>>>,
    // TODO: oneshot to close stream
}

pub trait IntoPart {
    fn into_part(self, name: Option<String>) -> BoxFuture<'static, Result<Part, anyhow::Error>>;
}

impl IntoPart for Vec<u8> {
    fn into_part(self, name: Option<String>) -> BoxFuture<'static, Result<Part, Error>> {
        Box::pin(async move {
            let file_name = name.unwrap_or(format!("default_filename{}.jpg", random::<u32>()));
            let part = Part::stream(self).file_name(file_name);
            dbg!(&part);
            Ok(part)
        })
    }
}

impl IntoPart for PathBuf {
    fn into_part(self, file_name: Option<String>) -> BoxFuture<'static, Result<Part, anyhow::Error>> {
        Box::pin(async move {
            let file_name = file_name.unwrap_or(self
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
            );

            let file = FramedRead::new(
                tokio::fs::File::open(self).await?,

                FileDecoder,
            );

            log::trace!("{}:{} PathBuf::into_part | return", file!(), line!());
            Ok::<Part, anyhow::Error>(Part::stream(Body::wrap_stream(file)).file_name(file_name))
        })
    }
}

impl LongpollStream {
    pub fn set_prefix(&mut self, prefix: impl AsRef<str>) -> &mut Self {
        self.prefixes.write().unwrap().push(prefix.as_ref().to_owned().to_lowercase());
        self
    }

    pub fn set_allowed_events(&mut self, mut events: &[LongpollEvent]) -> &mut Self {
        self.events.write().unwrap().append(&mut events.to_vec());
        self
    }

    pub(crate) fn new(inner: Receiver<LongpollUpdate>) -> Self {
        Self {
            inner,
            events: Arc::new(RwLock::new(vec![])),
            prefixes: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn build_stream(self) -> impl Stream<Item = LongpollUpdate> + Unpin {
        use LongpollUpdate::*;
        let inner = self.inner;
        let events = self.events;
        let prefixes = self.prefixes;
        let with_events = inner.filter_map(move |update| {
            let events = events.clone();

            Box::pin(async move {
                let events = events.read().unwrap();
                match &update {
                    // Alternatively for each other event
                    MessageNew {message} => {
                        if events.iter().any(|event| event == &LongpollEvent::MessageNew) {
                            Some(update)
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            })
        });
        let with_prefixes = with_events.filter_map(move |update| {
            let prefixes = prefixes.clone();

            Box::pin(async move {
                let prefixes = prefixes.read().unwrap();
                match &update {
                    MessageNew {message} => {
                        if let Some(prefix) = prefixes.iter().find(|&prefix| message.text.to_lowercase().starts_with(prefix)) {
                            let mut message = message.clone();
                            message.text = message.text.replace(prefix, "").trim().to_owned();
                            Some(MessageNew {message})
                        } else {
                            None
                        }
                    },
                    // We keep all other events
                    _ => Some(update)
                }
            })
        });

        // More handlers go here

        with_prefixes
    }
}

impl Stream for LongpollStream {
    type Item = LongpollUpdate;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}
