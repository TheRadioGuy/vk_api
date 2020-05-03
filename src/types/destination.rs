use Destination::*;
/// Used for uploading files
/// [docs](https://vk.com/dev/upload_files)
#[derive(Debug)]
pub enum Destination {
    /// Used for loading photo in album
    Album,
    /// Loading photo to wall
    Wall,
    /// Load photo as owner one
    OwnerPhoto,
    /// Load photo to messages
    Message,
    /// Load photo as chat photo
    ChatPhoto,
    /// Load photo as Market photo
    MarketPhoto,
    /// Load photo for album
    MarketAlbum,
    /// Load audio
    Audio,
    /// Load Video
    Video,
    /// Load Document in documents section
    Document,
    /// Load document on wall
    DocumentWall,
    /// Load Document in message
    DocumentMessage,
    /// Load photo as cover in group
    Cover,
    /// Load audio as Audio message
    AudioMessage,
    /// Story photo
    StoryPhoto,
    /// story video
    StoryVideo,
}

impl Destination {
    pub fn pick_method_load(&self) -> &'static str {
        match self {
            Album => "photos.getUploadServer",
            Wall => "photos.saveWallPhoto",
            OwnerPhoto => "photos.getOwnerPhotoUploadServer",
            Message => "photos.getMessagesUploadServer",
            ChatPhoto => "photos.getChatUploadServer",
            MarketPhoto => "photos.getMarketUploadServer",
            MarketAlbum => "photos.getMarketAlbumUploadServer",
            Audio => "audio.getUploadServer",
            Video => "video.save",
            DocumentWall => "docs.getWallUploadServer",
            DocumentMessage => "docs.getMessagesUploadServer",
            Document => "docs.getUploadServer",
            Cover => "photos.getOwnerCoverPhotoUploadServer",
            AudioMessage => "docs.getMessagesUploadServer",
            StoryPhoto => "stories.getPhotoUploadServer",
            StoryVideo => "stories.getVideoUploadServer",
        }
    }

    /// If it returns `none` - We dont have to save file(it's saved automatically)
    pub fn pick_method_save(&self) -> &'static str {
        match self {
            Album => "photos.save",
            Wall => "photos.saveWallPhoto",
            OwnerPhoto => "photos.saveOwnerPhoto",
            Message => "photos.saveMessagesPhoto",
            ChatPhoto => "photos.setChatPhoto",
            MarketPhoto => "photos.saveMarketPhoto",
            MarketAlbum => "photos.saveMarketAlbumPhoto",
            Audio => "audio.save",
            Video => "none",
            DocumentWall => "docs.save",
            DocumentMessage => "docs.save",
            Document => "docs.save",
            Cover => "photos.saveOwnerCoverPhoto",
            AudioMessage => "docs.save",
            StoryPhoto => "none",
            StoryVideo => "none",
        }
    }

    /// __TODO__: Replace `String` to `&str`
    pub fn pick_param(&self) -> String {
        match self {
            Album => "file1".to_owned(),
            Wall => "photo".to_owned(),
            OwnerPhoto => "photo".to_owned(),
            Message => "photo".to_owned(),
            ChatPhoto => "file".to_owned(),
            MarketPhoto => "file".to_owned(),
            MarketAlbum => "file".to_owned(),
            Audio => "file".to_owned(),
            Video => "video_file".to_owned(),
            Document => "file".to_owned(),
            DocumentWall => "file".to_owned(),
            DocumentMessage => "file".to_owned(),
            Cover => "photo".to_owned(),
            AudioMessage => "file".to_owned(),
            StoryPhoto => "file".to_owned(),
            StoryVideo => "video_file".to_owned(),
        }
    }
}
