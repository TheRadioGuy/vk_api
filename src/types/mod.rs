use crate::utils::bytes_from_response;
use crate::{Params, VK};
use anyhow::Context;
use bytes::Bytes;
use chrono::{serde::ts_seconds, DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub mod destination;
pub mod file;
pub mod keyboard;
pub mod longpoll;

#[derive(Deserialize, Debug)]
pub(crate) struct VkError {
    pub(crate) error_code: u16,
    pub(crate) error_msg: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub id: u32,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub peer_id: u32,
    pub conversation_message_id: u32,
    pub from_id: i32,
    pub text: String,
    pub attachments: Vec<Attachment>,
    pub reply_message: Option<Box<Message>>,
}

impl Message {
    pub async fn reply_with_photo(&self, vk: &VK, photo: Photo) -> Result<(), anyhow::Error> {
        let random_id: i64 = rand::random();
        let peer_id = self.peer_id;
        let reply_to = self.conversation_message_id;

        let mut params = Params::new();
        params.add_param("random_id", &random_id.to_string());
        params.add_param("peer_id", &peer_id.to_string());
        params.add_param("reply_to", &reply_to.to_string());
        let attach = format!("photo{}_{}", photo.owner_id, photo.id);
        let attach = match photo.access_token {
            Some(token) => format!("{}_{}", attach, token),
            None => attach,
        };
        params.add_param("attachment", &attach);

        let _: serde_json::Value = vk.request_post("messages.send", params).await?;

        Ok(())
    }

    pub async fn reply(&self, vk: &VK, text: Option<String>) -> Result<(), anyhow::Error> {
        let random_id: i64 = rand::random();
        let peer_id = self.peer_id;
        let reply_to = self.conversation_message_id;

        let mut params = Params::new();
        params.add_param("random_id", &random_id.to_string());
        params.add_param("peer_id", &peer_id.to_string());
        params.add_param("reply_to", &reply_to.to_string());

        if let Some(text) = text {
            params.add_param("message", &text);
        }

        let _: serde_json::Value = vk.request_post("messages.send", params).await?;

        Ok(())
    }

    pub async fn get_chat_context(&self, vk: &VK) -> Result<ConversationMembers, anyhow::Error> {
        let mut params = Params::new();
        params.add_param("peer_id", &self.peer_id.to_string());

        let response: serde_json::Value = vk
            .request("messages.getConversationMembers", &mut params)
            .await?;
        dbg!(&response);

        Ok(serde_json::from_value(
            // TODO make proper struct
            response["response"].to_owned(),
        )?)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Attachment {
    Photo { photo: Photo },

    // TODO: implement other variants of internally tagged enum via structs, not serde_json::Value
    Sticker { sticker: serde_json::Value },
    Video { video: serde_json::Value },
    Wall { wall: serde_json::Value },
    AudioMessage { audio_message: serde_json::Value },
    Doc { doc: serde_json::Value },
}

#[derive(Serialize, Debug, Clone)]
struct MessageRequest {
    user_id: Option<u32>,
    domain: Option<String>,
    chat_id: Option<u32>,
    user_ids: Option<String>,
    random_id: i64,
    peer_id: u32,
    // TODO: fill
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConversationMembers {
    pub count: u16,
    pub items: Vec<ChatUser>,
    pub profiles: Vec<User>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub is_closed: bool,
    pub can_access_closed: bool,
    pub screen_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChatUser {
    pub member_id: i32,
    pub invited_by: u32,
    #[serde(with = "ts_seconds")]
    pub join_date: DateTime<Utc>,
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub can_kick: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Photo {
    pub id: i32,
    pub access_token: Option<String>,
    pub album_id: i32,
    pub owner_id: i32,
    pub user_id: Option<u32>,
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub sizes: Vec<Size>,
    pub width: Option<u16>,
    pub height: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoSaveResponse {
    pub response: Vec<Photo>,
}

impl Photo {
    pub async fn get_photo_bytes(&self) -> Result<Bytes, anyhow::Error> {
        dbg!(&self.sizes);
        let sizes_sorted: Vec<Size> = self
            .sizes
            .clone()
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.width, &a.width))
            .sorted_by(|a, b| Ord::cmp(&b.height, &a.height))
            .collect();
        dbg!(&sizes_sorted);
        let url = sizes_sorted.first().unwrap().url.clone();

        let response: reqwest::Response = reqwest::get(&url).await.context(format!(
            "{}:{} Photo::get_photo | could not complete get request",
            file!(),
            line!()
        ))?;

        bytes_from_response(response).await
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Size {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoUploadServerResponse {
    pub response: InnerPhotoUploadServerResponse,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InnerPhotoUploadServerResponse {
    pub upload_url: String,
    pub album_id: i32,
    pub user_id: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoUploadResponse {
    pub server: u32,
    pub photo: String,
    pub hash: String,
}

#[cfg(test)]
mod tests {
    fn test_deserialize_longpoll_update() {
        let response = r#"
        {
          "ts": "10",
          "updates": [
            {
              "type": "message_new",
              "object": {
                "message": {
                  "date": 1590493480,
                  "from_id": 130047974,
                  "id": 0,
                  "out": 0,
                  "peer_id": 2000000002,
                  "text": "ля",
                  "conversation_message_id": 3,
                  "fwd_messages": [],
                  "important": false,
                  "random_id": 0,
                  "attachments": [],
                  "is_hidden": false
                },
                "client_info": {
                  "button_actions": [
                    "text",
                    "vkpay",
                    "open_app",
                    "location",
                    "open_link"
                  ],
                  "keyboard": true,
                  "inline_keyboard": true,
                  "lang_id": 0
                }
              },
              "group_id": 195707782,
              "event_id": "8f7910ab8a2de4a5c9b5226eba577fed3ca2548a"
            },
            {
              "type": "message_new",
              "object": {
                "message": {
                  "date": 1590493489,
                  "from_id": 130047974,
                  "id": 0,
                  "out": 0,
                  "peer_id": 2000000002,
                  "text": "",
                  "conversation_message_id": 4,
                  "fwd_messages": [],
                  "important": false,
                  "random_id": 0,
                  "attachments": [
                    {
                      "type": "photo",
                      "photo": {
                        "album_id": -7,
                        "date": 1589990975,
                        "id": 457263154,
                        "owner_id": 130047974,
                        "has_tags": false,
                        "access_key": "107e6a3e06c3b15a24",
                        "sizes": [
                          {
                            "height": 88,
                            "url": "https://sun9-26.userapi.com/hOtD_SLZAejjZy7x11XWHnY8H7NiHm7vgEtcXQ/HPA-Gw7qQd8.jpg",
                            "type": "m",
                            "width": 130
                          },
                          {
                            "height": 88,
                            "url": "https://sun9-26.userapi.com/hOtD_SLZAejjZy7x11XWHnY8H7NiHm7vgEtcXQ/HPA-Gw7qQd8.jpg",
                            "type": "o",
                            "width": 130
                          },
                          {
                            "height": 136,
                            "url": "https://sun9-25.userapi.com/KKuF4o2SLCeuGeiXHMOIbfLn2xAEuVJqLHNu9w/ziPf8FiLchk.jpg",
                            "type": "p",
                            "width": 200
                          },
                          {
                            "height": 217,
                            "url": "https://sun9-12.userapi.com/bSAVXCIEpvJydchRerjIcC7CgeAqKkFRK2PNMQ/1MgjXm45ojo.jpg",
                            "type": "q",
                            "width": 320
                          },
                          {
                            "height": 346,
                            "url": "https://sun9-62.userapi.com/kNBvfqB31GFRyaKXylgySJHHf18qal0GK9oyRQ/2gSmgznS1zM.jpg",
                            "type": "r",
                            "width": 510
                          },
                          {
                            "height": 51,
                            "url": "https://sun9-14.userapi.com/fFZ4HqvjfEXMPnO3MuvQPzydFXoAAVVxAIjQ_w/dYBqt4oyDY4.jpg",
                            "type": "s",
                            "width": 75
                          },
                          {
                            "height": 410,
                            "url": "https://sun9-13.userapi.com/itidL4PNlGrPSzAc3qf2rQWJZVZb4fDPAoNyYQ/uQJeofCHC0c.jpg",
                            "type": "x",
                            "width": 604
                          }
                        ],
                        "text": ""
                      }
                    }
                  ],
                  "is_hidden": false
                },
                "client_info": {
                  "button_actions": [
                    "text",
                    "vkpay",
                    "open_app",
                    "location",
                    "open_link"
                  ],
                  "keyboard": true,
                  "inline_keyboard": true,
                  "lang_id": 0
                }
              },
              "group_id": 195707782,
              "event_id": "ccd5a7e7ff7ca53dd6193d07ba2770c14a69ccba"
            }
          ]
        }
        "#;
    }
}
