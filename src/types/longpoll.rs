use crate::types::Message;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct InnerLongpollServerResponse {
    pub(crate) key: String,
    pub(crate) server: String,
    pub(crate) ts: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LongpollServerResponse {
    pub(crate) response: InnerLongpollServerResponse,
}

#[derive(Debug)]
pub(crate) enum LongpollResponse {
    Success(SuccessfulLongpollResponse),
    Fail(FailedLongpollResponse),
}

#[derive(Deserialize, Debug)]
pub(crate) struct FailedLongpollResponse {
    pub(crate) failed: u8,
    pub(crate) ts: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SuccessfulLongpollResponse {
    pub(crate) ts: String,
    pub(crate) updates: Vec<LongpollUpdate>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "object")]
#[serde(rename_all = "snake_case")]
pub enum LongpollUpdate {
    MessageNew { message: Message },

    // TODO: implement other variants
    MessageTypingState(serde_json::Value),
    MessageReply(serde_json::Value),
}
