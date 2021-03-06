use urlencoding::encode;
use serde::{Serialize};
use std::fmt;

#[derive(Debug, Serialize)]
#[serde(rename_all="lowercase")]
pub enum ButtonColor {
    Primary,
    Secondary,
    Negative,
    Positive
}

#[derive(Debug, Serialize)]
pub struct TextButton {
    r#type: String,
    label: String,
    payload: String,
}

impl TextButton {
    pub fn new<S1, S2>(label: S1, payload: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            r#type: "text".to_owned(),
            label: label.into(),
            payload: payload.into(),
        }
    }
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>
    {
        self.label = label.into();
        self
    }
    pub fn payload<S>(mut self, payload: S) -> Self
    where
        S: Into<String>
    {
        self.payload = payload.into();
        self
    }
}

impl From<TextButton> for Action {
    fn from(val: TextButton) -> Self {
        Self::Text(val)
    }
}

#[derive(Debug, Serialize)]
pub struct OpenLinkButton {
    r#type: String,
    link: String,
    label: String,
    payload: String,
}

impl OpenLinkButton {
    pub fn new<S1, S2, S3>(link: S1, label: S2, payload: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>
    {
        Self {
            r#type: "open_link".to_owned(),
            link: link.into(),
            label: label.into(),
            payload: payload.into(),
        }
    }
    pub fn link<S>(mut self, link: S) -> Self
    where
        S: Into<String>
    {
        self.link = link.into();
        self
    }
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>
    {
        self.label = label.into();
        self
    }
    pub fn payload<S>(mut self, payload: S) -> Self
    where
        S: Into<String>
    {
        self.payload = payload.into();
        self
    }
}

impl From<OpenLinkButton> for Action {
    fn from(val: OpenLinkButton) -> Self {
        Self::OpenLink(val)
    }
}

#[derive(Debug, Serialize)]
pub struct LocationButton {
    r#type: String,
    payload: String,
}

impl LocationButton {
    pub fn new<S>(payload: S) -> Self
    where
        S: Into<String>
    {
        Self {
            r#type: "location".to_owned(),
            payload: payload.into(),
        }
    }
    pub fn payload<S>(mut self, payload: S) -> Self
    where
        S: Into<String>
    {
        self.payload = payload.into();
        self
    }
}

impl From<LocationButton> for Action {
    fn from(val: LocationButton) -> Self {
        Self::Location(val)
    }
}

#[derive(Debug, Serialize)]
pub struct VKPayButton {
    r#type: String,
    payload: String,
    hash: String,
}

impl VKPayButton {
    pub fn new<S1, S2>(payload: S1, hash: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>
    {
        Self {
            r#type: "vkpay".to_owned(),
            payload: payload.into(),
            hash: hash.into(),
        }
    }
    pub fn payload<S>(mut self, payload: S) -> Self
    where
        S: Into<String>
    {
        self.payload = payload.into();
        self
    }
    pub fn hash<S>(mut self, hash: S) -> Self
    where
        S: Into<String>
    {
        self.hash = hash.into();
        self
    }
}

impl From<VKPayButton> for Action {
    fn from(val: VKPayButton) -> Self {
        Self::VKPay(val)
    }
}

#[derive(Debug, Serialize)]
pub struct VKAppsButton {
    r#type: String,

    app_id: i64,
    owner_id: i64,

    payload: String,
    label: String,
    hash: String,
}

impl VKAppsButton {
    pub fn new<S1, S2, S3>(app_id: i64, owner_id: i64, payload: S1, label: S2, hash: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>
    {
        Self {
            r#type: "open_app".to_owned(),

            app_id,
            owner_id,

            payload: payload.into(),
            label: label.into(),
            hash: hash.into(),
        }
    }
    pub fn app_id(mut self, app_id: i64) -> Self {
        self.app_id = app_id;
        self
    }
    pub fn owner_id(mut self, owner_id: i64) -> Self {
        self.owner_id = owner_id;
        self
    }
    pub fn payload<S>(mut self, payload: S) -> Self
    where
        S: Into<String>
    {
        self.payload = payload.into();
        self
    }
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>
    {
        self.label = label.into();
        self
    }
    pub fn hash<S>(mut self, hash: S) -> Self
    where
        S: Into<String>
    {
        self.hash = hash.into();
        self
    }
}

impl From<VKAppsButton> for Action {
    fn from(val: VKAppsButton) -> Self {
        Self::VKApps(val)
    }
}

#[derive(Debug, Serialize)]
pub struct CallbackButton {
    r#type: String,
    label: String,
    payload: String,
}

impl CallbackButton {
    pub fn new<S1, S2>(label: S1, payload: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>
    {
        Self {
            r#type: "text".to_owned(),
            label: label.into(),
            payload: payload.into(),
        }
    }
    pub fn payload<S>(mut self, payload: S) -> Self
    where
        S: Into<String>
    {
        self.payload = payload.into();
        self
    }
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<String>
    {
        self.label = label.into();
        self
    }
}

impl From<CallbackButton> for Action {
    fn from(val: CallbackButton) -> Self {
        Action::Callback(val)
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Action {
    Text(TextButton),
    OpenLink(OpenLinkButton),
    Location(LocationButton),
    VKPay(VKPayButton),
    VKApps(VKAppsButton),
    Callback(CallbackButton),
}

#[derive(Debug, Serialize)]
pub struct Button {
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ButtonColor>,
}

impl Button {
    fn validate(action: &Action, color: &Option<ButtonColor>) -> bool {
        match *color {
            None => true,
            Some(_) => {
                matches!(*action, Action::Text(_) | Action::Callback(_))
            }
        }
    }
    pub fn new<T>(action: T, color: Option<ButtonColor>) -> Result<Self, &'static str>
    where T:
        Into<Action>
    {
        let action = action.into();
        if Self::validate(&action, &color) {
            Ok(Self { action, color })
        } else {
            Err("Color can be used only with text or callback button")
        }
    }
    pub fn action<T>(mut self, action: T) -> Self
    where T:
        Into<Action>
    {
        let action = action.into();
        if Self::validate(&action, &self.color) {
            self.action = action;
        }
        self
    }
    pub fn color(mut self, color: Option<ButtonColor>) -> Self {
        if Self::validate(&self.action, &color) {
            self.color = color;
        }
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Keyboard {
    #[serde(skip_serializing_if = "Option::is_none")]
    one_time: Option<bool>,
    buttons: Vec<Vec<Button>>,
    inline: bool,
}

impl Keyboard {
    fn validate(inline: bool, one_time: &Option<bool>) -> bool {
        !(matches!(one_time, &Option::Some(_)) && inline)
    }
    pub fn new<I1, I2>(buttons: I1, inline: bool, one_time: Option<bool>) -> Result<Self, &'static str>
    where
        I1: Into<Vec<I2>>,
        I2: Into<Vec<Button>>
    {
        if Self::validate(inline, &one_time) {
            Ok(Self {
                one_time,
                buttons: buttons.into().into_iter().map(Into::into).collect(),
                inline,
            })
        } else {
            Err("One_time field is not available for inline keyboard")
        }
    }
    pub fn one_time(mut self, one_time: Option<bool>) -> Self {
        if Self::validate(self.inline, &one_time) {
            self.one_time = one_time;
        }
        self
    }
    pub fn buttons<I1, I2>(mut self, buttons: I1) -> Self
        where
        I1: Into<Vec<I2>>,
        I2: Into<Vec<Button>>
    {
        self.buttons = buttons.into().into_iter().map(Into::into).collect();
        self
    }
    pub fn append_row(mut self, buttons: Vec<Button>) -> Self {
        self.buttons.push(buttons);
        self
    }
    pub fn append_to_row(mut self, button: Button, index: usize) -> Self {
        match self.buttons.get_mut(index) {
            Some(buttons) => buttons.push(button),
            None => self.buttons.push(vec![button]),
        };
        self
    }
    pub fn inline(mut self, inline: bool) -> Self {
        if Self::validate(inline, &self.one_time) {
            self.inline = inline;
        }
        self
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", encode(&serde_json::to_string(&self).expect("Failed to serialize keyboard")))
    }
}
