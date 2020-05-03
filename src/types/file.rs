use crate::params::Params;
use crate::types::destination::Destination;
use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub destination: Destination,
    params: Params,
}

impl File {
    pub fn new(path: &str, destination: Destination) -> Self {
        let path = path.to_owned();
        let path = PathBuf::from(path);
        let mut params = Params::new();
        if matches!(destination, Destination::AudioMessage) {
            params.add("type", "audio_message");
        }
        Self {
            path,
            destination,
            params,
        }
    }
    pub fn add_param(&mut self, key: &str, value: &str) {
        self.params.add(key, value);
    }
}
