use std::sync::Arc;

use gdk::{Paintable, Texture};
use glib::Bytes;
use gtk::Image;
use tokio::sync::Semaphore;

use crate::api_client::AnixartClient;

static REQUEST_LIMITER: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(3)));

pub async fn download_image_async(
    url: &str,
) -> Result<gtk::gdk::Texture, Box<dyn std::error::Error>> {
    let _permit = REQUEST_LIMITER.clone().acquire_owned().await?;
    let client = AnixartClient::new();

    let response = client.client.get(url).send().await?;
    // let bytes = response.bytes().await?;

    // let texture = Texture::from_bytes(&Bytes::from(&bytes[..])).expect("Failed to create texture");
    let texture = Texture::from_resource("/kz/findmyname284/anixartd/img/logo_light.png");
    println!("Image downloaded");

    Ok(texture)
}

pub fn load_image(image: &Image, url: &str) {
    let image_clone = image.clone();
    let url = url.to_string();
    glib::spawn_future_local(async move {
        match download_image_async(&url).await {
            Ok(texture) => {
                glib::idle_add_local(move || {
                    image_clone.set_paintable(Some(&texture));
                    glib::ControlFlow::Break
                });
            }
            Err(e) => {
                eprintln!("Ошибка загрузки: {}", e);
                glib::idle_add_local(move || {
                    if let Some(placeholder) = create_placeholder() {
                        image_clone.set_paintable(Some(&placeholder));
                    }
                    glib::ControlFlow::Break
                });
            }
        };
    });
}

fn create_placeholder() -> Option<Paintable> {
    let width = 100;
    let height = 100;

    Some(Paintable::new_empty(width, height))
}
