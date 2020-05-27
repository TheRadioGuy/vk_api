use futures::{StreamExt, Stream};
use vkapi::types::longpoll::LongpollUpdate;
use vkapi::{LongpollEvent, Destination, VK};
use vkapi::types::{Attachment, Message};
use image::{GenericImage, GenericImageView};
use std::path::{PathBuf, Path};
use std::str::FromStr;
use futures::future::BoxFuture;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    pretty_env_logger::try_init().unwrap_or(());

    let mut vk_api = vkapi::VK::new("5.107".to_owned(), "ru".to_owned(), 195707782);
    let access_token = "0225b8abfd299f52395b8560db2ec4297157adcde29cbf2478a8faf81d73a2d9f4838252f5249426cd611".to_string();
    vk_api.set_access_token(
        access_token
            .clone(),
    );

    let mut stream = vk_api.init_stream(195707782, 15).await;

    stream.set_prefix("ирка");
    stream.set_allowed_events(&[LongpollEvent::MessageNew]);

    let mut stream = stream.build_stream();

    let commands: Vec<(String, Box<dyn Send + Sync + Fn(Arc<VK>, Message) -> BoxFuture<'static, anyhow::Result<()>>>)> = vec![
        ("кек".to_owned(), Box::new(kek_command)),
        ("лол".to_owned(), Box::new(lol_command)),
        ("инверт".to_owned(), Box::new(invert_command))
    ];

    let mut handler = BotMessageHandler::new(stream, commands, vk_api);

    handler.handle_commands().await.unwrap();
}

struct BotMessageHandler<S>
where
    S: Stream<Item = LongpollUpdate> + Unpin,
{
    inner: S,
    message_commands: Vec<(String, Box<dyn Send + Sync + Fn(Arc<VK>, Message) -> BoxFuture<'static, anyhow::Result<()>>>)>,
    vk_api: Arc<VK>
}

impl<S> BotMessageHandler<S>
where
    S: Stream<Item = LongpollUpdate> + Unpin
{
    fn new(inner: S, message_commands: Vec<(String, Box<dyn Send + Sync + Fn(Arc<VK>, Message) -> BoxFuture<'static, anyhow::Result<()>>>)>, vk_api: VK) -> Self {
        Self {
            inner,
            message_commands,
            vk_api: Arc::new(vk_api)
        }
    }

    async fn handle_commands(&mut self) -> anyhow::Result<()> {
        while let Some(update) = self.inner.next().await {
            let update: LongpollUpdate = update;
            use LongpollUpdate::*;
            use Attachment::*;

            match update {
                MessageNew {message} => {
                    self.handle_message_commands(message).await.unwrap();
                },
                _ => ()
            }
        }
        Ok(())
    }

    async fn handle_message_commands(&mut self, message: Message) -> anyhow::Result<()> {
        let (_, cb) = self.message_commands.iter().find(|(s, cb)| {
            message.text.to_lowercase().trim() == s
        }).unwrap();

        cb(self.vk_api.clone(), message).await
    }
}

fn kek_command(vk_api: Arc<VK>, message: Message) -> BoxFuture<'static, anyhow::Result<()>> {
    Box::pin(async move {
        match message.attachments.first() {
            Some(attachment) => {
                match attachment {
                    Attachment::Photo {photo} => {
                        let mut image = photo.get_photo().await?;
                        kek(&mut image);
                        let photo_response = vk_api.upload(image.to_bytes(), Destination::Message).await?;
                        let mut photo = vk_api.save_photo(photo_response).await?;
                        message.reply_with_photo(&vk_api, photo.response.remove(0)).await
                    },
                    _ => message.reply(&vk_api, Some("Бля дай пикчу".to_owned())).await
                }
            },
            None => message.reply(&vk_api, Some("Бля дай пикчу".to_owned())).await
        }
    })
}

fn lol_command(vk_api: Arc<VK>, message: Message) -> BoxFuture<'static, anyhow::Result<()>> {
    Box::pin(async move {
        match message.attachments.first() {
            Some(attachment) => {
                match attachment {
                    Attachment::Photo {photo} => {
                        let mut image = photo.get_photo().await?;
                        kek_lol(&mut image);
                        let photo_response = vk_api.upload(image.to_bytes(), Destination::Message).await?;
                        let mut photo = vk_api.save_photo(photo_response).await?;
                        message.reply_with_photo(&vk_api, photo.response.remove(0)).await
                    },
                    _ => message.reply(&vk_api, Some("Бля дай пикчу".to_owned())).await
                }
            },
            None => message.reply(&vk_api, Some("Бля дай пикчу".to_owned())).await
        }
    })
}

fn invert_command(vk_api: Arc<VK>, message: Message) -> BoxFuture<'static, anyhow::Result<()>> {
    Box::pin(async move {
        match message.attachments.first() {
            Some(attachment) => {
                match attachment {
                    Attachment::Photo {photo} => {
                        let mut img = photo.get_photo().await?;
                        image::imageops::invert(&mut img);
                        let photo_response = vk_api.upload(img.to_bytes(), Destination::Message).await?;
                        let mut photo = vk_api.save_photo(photo_response).await?;
                        message.reply_with_photo(&vk_api, photo.response.remove(0)).await
                    },
                    _ => message.reply(&vk_api, Some("Бля дай пикчу".to_owned())).await
                }
            },
            None => message.reply(&vk_api, Some("Бля дай пикчу".to_owned())).await
        }
    })
}

fn kek<I>(img: &mut I)
where
    I: GenericImage,
    <I as GenericImageView>::InnerImageView: GenericImage + 'static,
    <I as GenericImageView>::Pixel: 'static
{
    let (width, height) = img.dimensions();

    let mut left_half = img.view(width / 2, 0, width / 2, height).to_image();
    image::imageops::flip_horizontal_in_place(&mut left_half);
    image::imageops::overlay(img, &left_half, 0, 0);
}

fn kek_lol<I>(img: &mut I)
where
    I: GenericImage,
    <I as GenericImageView>::InnerImageView: GenericImage + 'static,
    <I as GenericImageView>::Pixel: 'static
{
    let (width, height) = img.dimensions();

    let mut right_half = img.view(0, 0, width / 2, height).to_image();
    image::imageops::flip_horizontal_in_place(&mut right_half);
    image::imageops::overlay(img, &right_half, width / 2, 0);
}
