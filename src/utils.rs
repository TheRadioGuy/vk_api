// Copy-pasted from https://github.com/teloxide/teloxide/blob/master/src/requests/utils.rs, thanks guys
use std::{borrow::Cow, path::PathBuf};

use bytes::{Bytes, BytesMut};
use reqwest::{multipart::Part, Body};
use tokio_util::codec::{Decoder, FramedRead};

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

pub async fn file_to_part(path_to_file: PathBuf) -> Part {
    let file_name = path_to_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let file = FramedRead::new(
        tokio::fs::File::open(path_to_file).await.unwrap(), /* TODO: this
                                                             * can
                                                             * cause panics */
        FileDecoder,
    );

    Part::stream(Body::wrap_stream(file)).file_name(file_name)
}

pub fn file_from_memory_to_part(data: Cow<'static, [u8]>, name: String) -> Part {
    Part::bytes(data).file_name(name)
}

pub fn make_params_for_saving_photo(
    params: &mut crate::params::Params,
    upload_info: json::JsonValue,
) {
    if !upload_info["server"].is_null() {
        params.add_param(
            "server",
            &upload_info["server"].as_u32().unwrap().to_string(),
        );
    }
    if !upload_info["aid"].is_null() {
        params.add_param("aid", upload_info["aid"].as_str().unwrap());
    }
    if !upload_info["photo"].is_null() {
        params.add_param("photo", upload_info["photo"].as_str().unwrap());
    }
    if !upload_info["hash"].is_null() {
        params.add_param("hash", upload_info["hash"].as_str().unwrap());
    }
    if !upload_info["photos_list"].is_null() {
        params.add_param("photos_list", upload_info["photos_list"].as_str().unwrap());
    }
    if !upload_info["response"].is_null() {
        params.add_param("file", upload_info["response"].as_str().unwrap());
    }
    if !upload_info["crop_data"].is_null() {
        params.add_param("crop_data", upload_info["crop_data"].as_str().unwrap());
    }
    if !upload_info["crop_hash"].is_null() {
        params.add_param("crop_hash", upload_info["crop_hash"].as_str().unwrap());
    }
    if !upload_info["audio"].is_null() {
        params.add_param("audio", upload_info["audio"].as_str().unwrap());
    }
}
